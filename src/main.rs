mod commandline;
mod config;
mod template;

use config::Config;
use template::Template;

use std::path::Path;

use anyhow::{Context, Result};
use clap::{Arg, Command};

fn main() -> Result<()> {
    let args = Command::new("project-builder")
        .version("0.1.0")
        .author("Aidan Grigg <aidangrigg02@gmail.com")
        .about("A simple tool to help create new projects from predefined templates.")
        .arg(
            Arg::new("template-dir")
                .help("Directory to source templates from")
                .short('t'),
        )
        .arg(
            Arg::new("project-dir")
                .value_name("PROJECT_DIR")
                .help("Directory of the new project")
                .required(true),
        )
        .get_matches();

    let config = Config::load()?;

    let template_dir = match args.get_one::<String>("template-dir") {
        Some(dir) => dir.to_string(),
        None => {
            if config.template_dir.is_some() {
                config.template_dir.unwrap()
            } else {
                commandline::read_input(Some(
                    "Please select the directory you would like to source your templates from (must be an absolute path): ",
                ))?
            }
        }
    };

    let template_dir: Vec<_> = std::fs::read_dir(&template_dir)
        .with_context(|| format!("Failed to read templates from {}", &template_dir))?
        .filter_map(|dir| {
            let path = dir.unwrap().path();
            if path.is_dir() {
                Some(String::from(path.to_str().unwrap()))
            } else {
                None
            }
        })
        .collect();

    let project_dir = args.get_one::<String>("project-dir").unwrap().to_string();

    let template = &template_dir[commandline::list_select(
        Some("Please select the template you would like to use: "),
        &template_dir,
    )?];

    let _t = Template::new(Path::new(template), Path::new(&project_dir));

    anyhow::Ok(())
}
