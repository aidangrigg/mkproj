use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use assert_cmd::Command;

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
    project_dir.child("a.txt").assert(predicate::str::contains("This is a test Hello, World!"));

    temp_dir.close()?;

    Ok(())
}
