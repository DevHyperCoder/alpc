use alpc::parse;

#[test]
fn number_to_str() {
    assert_eq!(vec!("ONE ", "TWO ", "TREE "), parse("123".to_string()));
}
