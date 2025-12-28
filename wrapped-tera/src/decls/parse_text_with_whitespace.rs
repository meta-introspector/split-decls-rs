macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! parse_text_with_whitespace {
    () => {
        deps!();
        # [test] fn parse_text_with_whitespace () { let ast = parse (" hello world ") . unwrap () ; assert_eq ! (ast [0] , Node :: Text (" hello world " . to_string ())) ; }
    };
}

parse_text_with_whitespace!();