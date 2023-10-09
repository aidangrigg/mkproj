use std::fs;
use std::path::{Path, PathBuf};
use std::str;

use crate::commandline;

use anyhow::{Context, Result};

enum PathDescriptor {
    File,
    Dir,
}

trait PathType {
    fn path_descriptor(&self) -> PathDescriptor;
}

impl PathType for PathBuf {
    fn path_descriptor(&self) -> PathDescriptor {
        if self.is_file() {
            PathDescriptor::File
        } else if self.is_dir() {
            PathDescriptor::Dir
        } else if self.is_symlink() {
            PathDescriptor::File
        } else {
            unreachable!()
        }
    }
}

struct Token {
    key: String,
    value: String,
}

impl Token {
    fn new(key: &str) -> Result<Self> {
        Ok(Self {
            key: String::from(key),
            value: commandline::read_input(Some(&format!("Please enter your {}: ", key)))?,
        })
    }
}

pub struct Template {
    tokens: Vec<Token>,
}

impl Template {
    pub fn new(template_path: &Path, project_path: &Path) -> Result<Self> {
        let mut template = Self { tokens: Vec::new() };

        template.build(template_path, project_path)?;

        Ok(template)
    }

    fn build(&mut self, template_path: &Path, destination: &Path) -> Result<()> {
        fs::create_dir_all(destination)?;

        let template_files: Vec<PathBuf> = fs::read_dir(template_path)
            .with_context(|| {
                format!(
                    "Files from directory {} could not be read",
                    template_path.to_str().unwrap()
                )
            })?
            .map(|file| match file {
                Ok(val) => Ok(val.path()),
                Err(_) => Err(anyhow::anyhow!("Error parsing file")),
            })
            .collect::<Result<Vec<PathBuf>>>()?;

        for template_file in template_files {
            let file_name = template_file.file_name().unwrap();

            match template_file.path_descriptor() {
                PathDescriptor::File => {
                    let file_contents = fs::read_to_string(&template_file)?;
                    let file_contents = self.parse_file(file_contents)?;

                    fs::write(destination.join(file_name), file_contents).with_context(|| {
                        format!(
                            "Could not write to file {}",
                            destination.join(file_name).to_str().unwrap()
                        )
                    })?;
                }
                PathDescriptor::Dir => {
                    self.build(&template_path.join(file_name), &destination.join(file_name))?;
                }
            }
        }

        Ok(())
    }

    fn parse_file(&mut self, file_contents: String) -> Result<String> {
        file_contents
            .lines()
            .map(|line| Ok(self.parse_line(line)? + "\n"))
            .collect()
    }

    fn parse_line(&mut self, line: &str) -> Result<String> {
        let token_positions: Vec<(usize, usize)> = line
            .match_indices("$*{")
            .filter_map(|start| {
                let start_pos = start.0;
                let end_pos = line[start.0..].find("}*");

                end_pos.map(|end_pos| (start_pos, end_pos + start_pos + 2))
            })
            .collect();

        let mut parsed_line = String::from(line);

        for token_pos in token_positions {
            let token_name = &parsed_line[token_pos.0..token_pos.1];
            let token = self.find_token(token_name)?;
            parsed_line.replace_range(token_pos.0..token_pos.1, &token.value);
        }

        Ok(parsed_line)
    }

    fn find_token(&mut self, key: &str) -> Result<&Token> {
        match self.tokens.iter().position(|token| token.key == key) {
            Some(idx) => Ok(&self.tokens[idx]),
            None => self.create_token(key),
        }
    }

    fn create_token(&mut self, key: &str) -> Result<&Token> {
        let t = Token::new(key)?;
        self.tokens.push(t);
        Ok(self.tokens.last().unwrap()) // i feel like there should be a better way than doing this
    }
}
