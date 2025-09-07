use magic_cli::commands::json::validate_json_query;

#[test]
fn validate_json_query_ok() {
    let ok = validate_json_query("user.0.name").unwrap();
    assert_eq!(ok, "user.0.name");
}

#[test]
fn validate_json_query_err() {
    let err = validate_json_query("").unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("不能为空") || msg.contains("empty"));
}


