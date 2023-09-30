use std::io::{self, Write};

use anyhow::{Context, Result};

pub fn read_input(prompt: Option<&str>) -> Result<String> {
    if let Some(prompt) = prompt {
        println!("{}", prompt);
        io::stdout().flush().ok();
    }

    let stdio = io::stdin();
    let mut project_name = String::new();

    stdio
        .read_line(&mut project_name)
        .context("Could not read from stdin")?;

    Ok(String::from(project_name.trim()))
}

pub fn list_select(prompt: Option<&str>, list: &Vec<String>) -> Result<usize> {
    if let Some(prompt) = prompt {
        println!("{}", prompt);
        io::stdout().flush().ok();
    }

    for (idx, row) in list.iter().enumerate() {
        println!("{}: {}", idx + 1, &row);
    }

    let mut selected_row: usize = parse_int(
        "Please enter a row: ",
        "Row must be an integer value, please try again",
    )?;

    loop {
        if selected_row >= 1 && selected_row <= list.len() {
            break Ok(selected_row - 1);
        }

        selected_row = parse_int(
            "Selected row is not in the list, please try again: ",
            "Row must be an integer value, please try again",
        )?;
    }
}

fn parse_int(prompt: &str, error_message: &str) -> Result<usize> {
    loop {
        match read_input(Some(prompt))?.parse::<usize>() {
            Ok(row) => break Ok(row),
            Err(_) => println!("{}", error_message),
        }
    }
}
