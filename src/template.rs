use std::fs;
use std::path::{Path, PathBuf};
use std::str;

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
    fn new(key: &str) -> Self {
        Self {
            key: String::from(key),
            value: String::from("unimplemented"),
        }
    }
}

pub struct Template {
    tokens: Vec<Token>,
}

impl Template {
    pub fn new(template_path: &Path, project_path: &Path) -> Self {
        let mut template = Self {
            tokens: Vec::new()
        };

        template.build(template_path, project_path);

        template
    }
    
    fn build(&mut self, template_path: &Path, destination: &Path) {
        fs::create_dir(destination).ok();

        let template_files: Vec<PathBuf> = match fs::read_dir(template_path) {
            Ok(val) => val,
            Err(err) => panic!("Error: {}", err.kind()),
        }
        .map(|f| match f {
            Ok(val) => val.path(),
            Err(err) => panic!("Error: {}", err.kind()),
        })
        .collect();

        for template_file in template_files {
            let file_name = template_file.file_name().unwrap();

            match template_file.path_descriptor() {
                PathDescriptor::File => {
                    let file_contents = match fs::read_to_string(&template_file) {
                        Ok(val) => self.parse_file(val),
                        Err(_) => panic!("Could not read contents of template file"),
                    };

                    match fs::write(destination.join(file_name), file_contents) {
                        Ok(_) => (),
                        Err(e) => panic!("Could not write file {}", e.kind()),
                    };
                }
                PathDescriptor::Dir => {
                    self.build(&template_path.join(file_name), &destination.join(file_name))
                }
            }
        }
    }

    fn parse_file(&mut self, file_contents: String) -> String {
        file_contents.lines().map(|line| self.parse_line(line) + "\n").collect()
    }

    fn parse_line(&mut self, line: &str) -> String {
        let token_positions: Vec<(usize, usize)> = line
            .match_indices("$*{")
            .filter_map(|start| {
                let start_pos = start.0;
                let end_pos = line[start.0..].find("}*");

                match end_pos {
                    Some(end_pos) => Some((start_pos, end_pos + 2)),
                    None => None,
                }
            })
            .collect();

        let mut parsed_line = String::from(line);

        for token_pos in token_positions {
            let token_name = &parsed_line[token_pos.0..token_pos.1];
            let token = self.find_token(token_name);
            parsed_line.replace_range(token_pos.0..token_pos.1, &token.value);
        }

        return parsed_line;
    }

    fn find_token(&mut self, key: &str) -> &Token {
        if self.tokens.iter().any(|token| token.key == key) {
            // looping through the list of tokens twice here kinda sucks
            return self.tokens.iter().find(|token| token.key == key).unwrap();
        } else {
            return self.create_token(key);
        }
    }

    fn create_token(&mut self, key: &str) -> &Token {
        let t = Token::new(key);
        self.tokens.push(t);
        self.tokens.last().unwrap() // i feel like there should be a better way than doing this
    }
}
