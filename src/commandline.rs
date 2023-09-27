use std::io::{self, Write};

pub fn read_input(prompt: Option<&str>) -> String {

    match prompt {
        Some(prompt) => {
            print!("{}", prompt);
            io::stdout().flush().ok();
        },
        None => (),
    };

    let stdio = io::stdin();
    let mut project_name = String::new();

    stdio.read_line(&mut project_name).expect("Could not read from stdin");

    String::from(project_name.trim())
}
