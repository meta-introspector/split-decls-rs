macro_rules! deps {
    () => {
        WS!();
        Node!();
    };
}

macro_rules! parse_comment_tag {
    () => {
        deps!();
        fn parse_comment_tag (pair : Pair < Rule >) -> Node { let mut ws = WS :: default () ; let mut content = String :: new () ; for p in pair . into_inner () { match p . as_rule () { Rule :: comment_start => { ws . left = p . as_span () . as_str () == "{#-" ; } Rule :: comment_end => { ws . right = p . as_span () . as_str () == "-#}" ; } Rule :: comment_text => { content = p . as_str () . to_owned () ; } _ => unreachable ! () , } ; } Node :: Comment (ws , content) }
    };
}

parse_comment_tag!()