mod commandline;
mod template;

use template::Template;

use commandline::Args;
use std::{collections::HashSet, env, path::Path};

fn main() {
    let args: HashSet<Args> = commandline::parse_args(env::args().collect());
    let mut template_dir = String::new();
    let mut project_dir = String::new();

    for arg in args {
        match arg {
            Args::TemplateDir(dir) => template_dir = dir,
            Args::ProjectDir(dir) => project_dir = dir,
        }
    }

    if template_dir.is_empty() {
        template_dir = commandline::read_input(Some(
            "Please select the directory you would like to source your templates from: ",
        ));
    }

    if project_dir.is_empty() {
        project_dir =
            commandline::read_input(Some("Please enter the directory of your new project: "));
    }

    let template_dir: Vec<_> = std::fs::read_dir(&template_dir)
        .expect("Could not read template directory")
        .filter_map(|dir| {
            let path = dir.unwrap().path();
            if path.is_dir() {
                Some(String::from(path.to_str().unwrap()))
            } else {
                None
            }
        })
        .collect();

    let template = &template_dir[commandline::list_select(
        Some("Please select the template you would like to use: "),
        &template_dir,
    )];

    let _t = Template::new(Path::new(template), Path::new(&project_dir));
}
