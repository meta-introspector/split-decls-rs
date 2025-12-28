macro_rules! deps {
    () => {
        WS!();
        Node!();
    };
}

macro_rules! parse_extends {
    () => {
        deps!();
        fn parse_extends (pair : Pair < Rule >) -> Node { let mut ws = WS :: default () ; let mut file = None ; for p in pair . into_inner () { match p . as_rule () { Rule :: tag_start => { ws . left = p . as_span () . as_str () == "{%-" ; } Rule :: string => file = Some (replace_string_markers (p . as_span () . as_str ())) , Rule :: tag_end => { ws . right = p . as_span () . as_str () == "-%}" ; } _ => unreachable ! () , } ; } Node :: Extends (ws , file . unwrap ()) }
    };
}

parse_extends!()