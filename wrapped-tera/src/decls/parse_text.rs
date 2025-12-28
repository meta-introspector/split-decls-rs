macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! parse_text {
    () => {
        deps!();
        # [test] fn parse_text () { let ast = parse ("hello world") . unwrap () ; assert_eq ! (ast [0] , Node :: Text ("hello world" . to_string ())) ; }
    };
}

parse_text!();