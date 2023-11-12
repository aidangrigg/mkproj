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

    // If a config-path is passed in through the command line, use it.
    // Otherwise, use the path passed into the function
    let config_path = if let Some(cfg) = args.get_one::<String>("config-path") {
        Path::new(cfg)
    } else {
        config_path
    };

    let mut config = Config::load(config_path)?;

    // If a template-dir is passed in through the command line, use it.
    // Otherwise, check if a template-dir has been specified in the users config file.
    // Otherwise, prompt the user to input a template directory
    let template_dir = if let Some(dir) = args.get_one::<String>("template-dir") {
        dir.to_string()
    } else if let Some(dir) = config.template_dir {
        dir
    } else {
        // if the user doesn't pass in a template directory, or have one already
        // set in their config, prompt them for one
        let input_directory = loop {
            let user_input = commandline::read_input(Some(
                "Please select the directory you would like to source your templates from (must be an absolute path): ",
            ))?;

            if Path::new(&user_input).exists() {
                break user_input;
            } else {
                println!("Directory does not exist. Must select an existing directory. Try again.");
            }
        };

        // let the user save their selected directory to the config file
        if commandline::yes_or_no(Some(
            "Would you like to save this template directory (so you don't have to input it again later)? (y or n): ",
        ))? {
            config.template_dir = Some(input_directory.clone());
            config.save(config_path)?
        }

        input_directory
    };

    // Get all the templates in the template directory
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

    // Get the passed in project directory
    let project_dir = args.get_one::<String>("project-dir").unwrap().to_string();

    // As the user what template they would like to use.
    let selected_template = &templates[commandline::list_select(
        Some("Please select the template you would like to use: "),
        &templates,
    )?];

    // Generate the template in the project directory.
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
        .arg(
            Arg::new("config-path")
                .help("Path to your configuration file")
                .short('c'),
        )
}
