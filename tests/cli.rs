use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions

#[test]
fn can_substitute_symbol() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = assert_fs::TempDir::new()?;

    let template_dir = temp_dir.child("templates/");
    let template = template_dir.child("something/");

    template.create_dir_all()?;

    template
        .child("a.txt")
        .write_str("This is a test $*{project-name}*")?;

    let project_dir = temp_dir.child("project/");

    let mut cmd = Command::cargo_bin("mkproj")?;

    cmd.arg("-t")
        .arg(template_dir.path())
        .arg(project_dir.path())
        .write_stdin("1\nHello, World!")
        .assert()
        .success();

    project_dir.child("a.txt").assert(predicate::path::exists());
    project_dir
        .child("a.txt")
        .assert(predicate::str::contains("This is a test Hello, World!"));

    temp_dir.close()?;

    Ok(())
}

#[test]
fn template_dir_no_arg() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = assert_fs::TempDir::new()?;

    let template_dir = temp_dir.child("templates/");
    let template = template_dir.child("something/");

    template.create_dir_all()?;

    template.child("a.txt").write_str("This is a test")?;

    let project_dir = temp_dir.child("project/");

    let config_path = temp_dir.child(".config/").child("mkproj");

    config_path.create_dir_all()?;

    let config_file = config_path.child("config.toml");

    config_file.touch()?;

    let mut cmd = Command::cargo_bin("mkproj")?;

    cmd.arg("-c")
        .arg(config_file.path())
        .arg(project_dir.path())
        .write_stdin(format!("{}\nn\n1\n", template_dir.path().to_str().unwrap()))
        .assert()
        .success();

    project_dir.child("a.txt").assert(predicate::path::exists());
    project_dir
        .child("a.txt")
        .assert(predicate::str::contains("This is a test"));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn template_dir_config() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = assert_fs::TempDir::new()?;

    let template_dir = temp_dir.child("templates/");
    let template = template_dir.child("something/");

    template.create_dir_all()?;

    template.child("a.txt").write_str("This is a test")?;

    let project_dir = temp_dir.child("project/");

    let config_path = temp_dir.child(".config/").child("mkproj");

    config_path.create_dir_all()?;

    let config_file = config_path.child("config.toml");

    println!("here");

    config_file
        .write_str(&format!("template_dir = \"{}\"", template_dir.path().to_str().unwrap()))?;

    println!("here2");

    let mut cmd = Command::cargo_bin("mkproj")?;

    cmd.arg("-c")
        .arg(config_file.path())
        .arg(project_dir.path())
        .write_stdin("1")
        .assert()
        .success();

    project_dir.child("a.txt").assert(predicate::path::exists());
    project_dir
        .child("a.txt")
        .assert(predicate::str::contains("This is a test"));

    println!("here3");
    temp_dir.close()?;
    Ok(())
}

#[test]
fn template_dir_arg() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = assert_fs::TempDir::new()?;

    let template_dir = temp_dir.child("templates/");
    let template = template_dir.child("something/");

    template.create_dir_all()?;

    template
        .child("a.txt")
        .write_str("This is a test")?;

    let project_dir = temp_dir.child("project/");

    let mut cmd = Command::cargo_bin("mkproj")?;

    cmd.arg("-t")
        .arg(template_dir.path())
        .arg(project_dir.path())
        .write_stdin("1")
        .assert()
        .success();

    project_dir.child("a.txt").assert(predicate::path::exists());
    project_dir
        .child("a.txt")
        .assert(predicate::str::contains("This is a test"));

    temp_dir.close()?;
    Ok(())
}
