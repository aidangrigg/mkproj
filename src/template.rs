use std::fs;
use std::path::{Path, PathBuf};

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
        } else {
            PathDescriptor::File
        }
    }
}

pub fn build_template(template_path: &Path, destination: &Path) {
    fs::create_dir(destination).ok();

    let template_files = match fs::read_dir(template_path) {
        Ok(val) => val,
        Err(err) => panic!("Error: {}", err.kind()),
    }
    .map(|f| match f {
        Ok(val) => val.path(),
        Err(err) => panic!("Error: {}", err.kind()),
    });
    for template_file in template_files {
        let file_name = match template_file.file_name() {
            Some(val) => val,
            None => {
                println!("skipping file {}", template_file.to_str().unwrap());
                continue;
            }
        };

        match template_file.path_descriptor() {
            PathDescriptor::File => {
                let file_contents = match fs::read(&template_file) {
                    Ok(val) => val,
                    Err(_) => panic!("Could not read contents of template file"),
                };

                match fs::write(destination.join(file_name), file_contents) {
                    Ok(_) => (),
                    Err(e) => panic!("Could not write file {}", e.kind()),
                };
            }
            PathDescriptor::Dir => {
                build_template(&template_path.join(file_name), &destination.join(file_name))
            }
        }
    }
}
