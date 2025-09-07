use assert_cmd::Command;

#[test]
fn json_pretty_prints() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("magic_cli")?;
    cmd.arg("json");

    let input = r#"{"a":1}"#;
    cmd.write_stdin(input)
        .assert()
        .success();

    Ok(())
}


