macro_rules! deps {
    () => {
        Node!();
        WS!();
    };
}

macro_rules! parse_raw_tag {
    () => {
        deps!();
        fn parse_raw_tag (pair : Pair < Rule >) -> Node { let mut start_ws = WS :: default () ; let mut end_ws = WS :: default () ; let mut text = None ; for p in pair . into_inner () { match p . as_rule () { Rule :: raw_tag => { for p2 in p . into_inner () { match p2 . as_rule () { Rule :: tag_start => start_ws . left = p2 . as_span () . as_str () == "{%-" , Rule :: tag_end => start_ws . right = p2 . as_span () . as_str () == "-%}" , _ => unreachable ! () , } } } Rule :: raw_text => text = Some (p . as_str () . to_string ()) , Rule :: endraw_tag => { for p2 in p . into_inner () { match p2 . as_rule () { Rule :: tag_start => end_ws . left = p2 . as_span () . as_str () == "{%-" , Rule :: tag_end => end_ws . right = p2 . as_span () . as_str () == "-%}" , _ => unreachable ! () , } } } _ => unreachable ! ("unexpected {:?} rule in parse_raw_tag" , p . as_rule ()) , } ; } Node :: Raw (start_ws , text . unwrap () , end_ws) }
    };
}

parse_raw_tag!()