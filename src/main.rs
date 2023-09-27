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
            "Please enter the directory of the template you would like to use: ",
        ));
    }

    if project_dir.is_empty() {
        project_dir =
            commandline::read_input(Some("Please enter the directory of your new project: "));
    }

    let _t = Template::new(Path::new(&template_dir), Path::new(&project_dir));
}
