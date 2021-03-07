use alpc::parse;

#[test]
fn test_upper_case_output() {
    assert_eq!(
        vec!("ALPHA ", "BRAVO ", "CHARLIE "),
        parse("ABC".to_string())
    );
}

#[test]
fn test_lowercase_input() {
    assert_eq!(vec!("a", "s", "d", "f"), parse("asdf".to_string()));
}

#[test]
fn test_mixed_input() {
    assert_eq!(
        vec!(
            "NOVEMBER ",
            "WHISKEY ",
            "SIERRA ",
            "ALPHA ",
            "TWO ",
            "NINER ",
            "ZERO ",
            " ",
            "c",
            "l",
            "i",
            "m",
            "b",
            " ",
            "t",
            "o",
            " ",
            "f",
            "l",
            " ",
            "ZERO ",
            "EIGHT ",
            "ZERO "
        ),
        parse("NWSA290 climb to fl 080".to_string())
    );
}

#[test]
fn test_space() {
    assert_eq!(
        vec!("a", "s", " ", "d", "f", "ONE ", " "),
        parse("as df1 ".to_string())
    );
}
