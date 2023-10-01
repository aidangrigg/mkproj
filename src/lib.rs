mod commandline;
mod config;
mod template;

use config::Config;
use template::Template;

use std::path::Path;

use anyhow::{Context, Result};
use clap::{Arg, Command};

pub fn run(config_path: &Path) -> Result<()> {
    let args = project_arguments().get_matches();
    let config = Config::load(config_path)?;

    let template_dir = if let Some(dir) = args.get_one::<String>("template-dir") {
        dir.to_string()
    } else if let Some(dir) = config.template_dir {
        dir
    } else {
        commandline::read_input(Some(
            "Please select the directory you would like to source your templates from (must be an absolute path): ",
        ))?
    };

    let templates: Vec<_> = std::fs::read_dir(&template_dir)
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

    let selected_template = &templates[commandline::list_select(
        Some("Please select the template you would like to use: "),
        &templates,
    )?];

    let _ = Template::new(Path::new(selected_template), Path::new(&project_dir));

    Ok(())
}

pub fn project_arguments() -> clap::Command {
    Command::new("mkproj")
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
}
