macro_rules! deps {
    () => {
        FunctionCall!();
        Node!();
        WS!();
        FilterSection!();
    };
}

macro_rules! parse_filter_section {
    () => {
        deps!();
        fn parse_filter_section (pair : Pair < Rule >) -> TeraResult < Node > { let mut start_ws = WS :: default () ; let mut end_ws = WS :: default () ; let mut filter = None ; let mut body = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: filter_tag => { for p2 in p . into_inner () { match p2 . as_rule () { Rule :: tag_start => start_ws . left = p2 . as_span () . as_str () == "{%-" , Rule :: tag_end => start_ws . right = p2 . as_span () . as_str () == "-%}" , Rule :: fn_call => filter = Some (parse_fn_call (p2) ?) , Rule :: ident => { filter = Some (FunctionCall { name : p2 . as_str () . to_string () , args : HashMap :: new () , }) ; } _ => unreachable ! ("Got {:?} while parsing filter_tag" , p2) , } } } Rule :: content | Rule :: macro_content | Rule :: block_content | Rule :: filter_section_content | Rule :: for_content => { body . extend (parse_content (p) ?) ; } Rule :: endfilter_tag => { for p2 in p . into_inner () { match p2 . as_rule () { Rule :: tag_start => end_ws . left = p2 . as_span () . as_str () == "{%-" , Rule :: tag_end => end_ws . right = p2 . as_span () . as_str () == "-%}" , _ => unreachable ! () , } } } _ => unreachable ! ("unexpected {:?} rule in parse_filter_section" , p . as_rule ()) , } ; } Ok (Node :: FilterSection (start_ws , FilterSection { filter : filter . unwrap () , body } , end_ws)) }
    };
}

parse_filter_section!();