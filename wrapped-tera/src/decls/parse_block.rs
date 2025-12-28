macro_rules! deps {
    () => {
        Block!();
        WS!();
        Node!();
    };
}

macro_rules! parse_block {
    () => {
        deps!();
        fn parse_block (pair : Pair < Rule >) -> TeraResult < Node > { let mut start_ws = WS :: default () ; let mut end_ws = WS :: default () ; let mut name = None ; let mut body = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: block_tag => { for p2 in p . into_inner () { match p2 . as_rule () { Rule :: tag_start => start_ws . left = p2 . as_span () . as_str () == "{%-" , Rule :: tag_end => start_ws . right = p2 . as_span () . as_str () == "-%}" , Rule :: ident => name = Some (p2 . as_span () . as_str () . to_string ()) , _ => unreachable ! () , } ; } } Rule :: block_content => body . extend (parse_content (p) ?) , Rule :: endblock_tag => { for p2 in p . into_inner () { match p2 . as_rule () { Rule :: tag_start => end_ws . left = p2 . as_span () . as_str () == "{%-" , Rule :: tag_end => end_ws . right = p2 . as_span () . as_str () == "-%}" , Rule :: ident => () , _ => unreachable ! () , } ; } } _ => unreachable ! ("unexpected {:?} rule in parse_filter_section" , p . as_rule ()) , } ; } Ok (Node :: Block (start_ws , Block { name : name . unwrap () , body } , end_ws)) }
    };
}

parse_block!()