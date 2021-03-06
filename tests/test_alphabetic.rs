use alpc::parse;

#[test]
fn test_upper_case_output() {
    assert_eq!(
        vec!("ALPHA ", "BRAVO ", "CHARLIE "),
        parse("abC".to_string())
    );
}
