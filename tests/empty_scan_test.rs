extern crate markdown_scanner;

#[test]
fn test_empty() {
    let result = markdown_scanner::scan("".to_string());
    assert_eq!(result.len(), 0);
}

