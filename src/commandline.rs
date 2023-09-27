use std::{io::{self, Write}, collections::HashSet};

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Args {
    TemplateDir (String),
    ProjectDir (String),
}

pub fn parse_args(args: Vec<String>) -> HashSet<Args> {

    let mut parsed_args: HashSet<Args> = HashSet::new();

    let mut args = args.iter();
    args.next();

    for arg in args {
        if arg.starts_with("-t") {
            let template_dir = arg.strip_prefix("-t").unwrap();

            if template_dir.len() > 0  {
                parsed_args.insert(Args::TemplateDir(String::from(template_dir)));
            } else {
                fatal_error(2, 
                    "-t prefix used incorrectly. Correct syntax is \"-t<TEMPLATE-DIR> (without a space)\""
                );
            }
        } else if arg.starts_with("-") {
            fatal_error(1, &(String::from(arg) + " is not supported at this time"));
        } else {
            parsed_args.insert(Args::ProjectDir(String::from(arg)));
        }
    }

    parsed_args
}

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

pub fn non_fatal_error(message: String, retry_attempts: u32, retry_fn: &dyn Fn()) {
    unimplemented!();
}

pub fn fatal_error(code: i32, message: &str) {
    print!("{}", message);
    io::stdout().flush().ok();

    std::process::exit(code);
}
