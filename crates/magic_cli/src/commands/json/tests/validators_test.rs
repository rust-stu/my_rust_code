use super::super::*;

#[test]
fn test_validate_json_query_ok_with_keys() {
    let q = "user.profile_name-1";
    assert_eq!(validate_json_query(q).unwrap(), q);
}

#[test]
fn test_validate_json_query_ok_with_indexes_and_keys() {
    let q = "users.0.name";
    assert_eq!(validate_json_query(q).unwrap(), q);
}

#[test]
fn test_validate_json_query_ok_only_indexes() {
    let q = "0.1.2";
    assert_eq!(validate_json_query(q).unwrap(), q);
}

#[test]
fn test_validate_json_query_err_empty() {
    let err = validate_json_query("").unwrap_err().to_string();
    assert!(err.contains("查询路径不能为空"));
}

#[test]
fn test_validate_json_query_err_contains_double_dot() {
    let err = validate_json_query("a..b").unwrap_err().to_string();
    assert!(err.contains("查询路径不能包含连续的点"));
}

#[test]
fn test_validate_json_query_err_invalid_part() {
    let err = validate_json_query("user.$name").unwrap_err().to_string();
    assert!(err.contains("无效的查询路径部分"));
}

#[test]
fn test_validate_json_query_err_trailing_dot_creates_empty_part() {
    let err = validate_json_query("user.").unwrap_err().to_string();
    assert!(err.contains("无效的查询路径部分"));
}

#[test]
fn test_is_valid_object_key_empty_false() {
    assert!(!is_valid_object_key(""));
}

#[test]
fn test_is_valid_object_key_alphanumeric_true() {
    assert!(is_valid_object_key("abc"));
    assert!(is_valid_object_key("abc123"));
    assert!(is_valid_object_key("A1b2C3"));
}

#[test]
fn test_is_valid_object_key_underscore_and_hyphen_true() {
    assert!(is_valid_object_key("_"));
    assert!(is_valid_object_key("-"));
    assert!(is_valid_object_key("a_b-c1"));
    assert!(is_valid_object_key("__--"));
}

#[test]
fn test_is_valid_object_key_invalid_chars_false() {
    assert!(!is_valid_object_key("a.b"));
    assert!(!is_valid_object_key("a b"));
    assert!(!is_valid_object_key("$"));
    assert!(!is_valid_object_key("a$"));
    assert!(!is_valid_object_key("!"));
}

#[test]
fn test_is_valid_object_key_unicode_alphanumeric_true() {
    // Rust 的 is_alphanumeric 支持 Unicode 字母数字
    assert!(is_valid_object_key("测试"));
    assert!(is_valid_object_key("ñ"));
    assert!(is_valid_object_key("β3"));
    assert!(is_valid_object_key("١٢٣"));
}