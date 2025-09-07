use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn json_query_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("magic_cli")?;
    cmd.args(["json", "user.name"]);

    let input = r#"{"user":{"name":"Alice","age":30}}"#;
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Alice\""));

    Ok(())
}


