macro_rules! deps {
    () => {
        WS!();
        Node!();
    };
}

macro_rules! parse_break_tag {
    () => {
        deps!();
        fn parse_break_tag (pair : Pair < Rule >) -> Node { let mut ws = WS :: default () ; for p in pair . into_inner () { match p . as_rule () { Rule :: tag_start => { ws . left = p . as_span () . as_str () == "{%-" ; } Rule :: tag_end => { ws . right = p . as_span () . as_str () == "-%}" ; } _ => unreachable ! () , } ; } Node :: Break (ws) }
    };
}

parse_break_tag!();