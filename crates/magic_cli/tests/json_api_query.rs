use magic_cli::commands::json::{query_json_path};
use serde_json::json;

#[test]
fn query_json_path_basic() {
    let v = json!({
        "user": {"name": "Alice", "age": 30},
        "items": [1, 2, 3]
    });

    let name = query_json_path(&v, "user.name").unwrap();
    assert_eq!(name, json!("Alice"));

    let second = query_json_path(&v, "items.1").unwrap();
    assert_eq!(second, json!(2));
}


