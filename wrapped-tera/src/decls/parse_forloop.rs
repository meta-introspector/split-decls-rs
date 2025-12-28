macro_rules! deps {
    () => {
        Forloop!();
        WS!();
        Node!();
    };
}

macro_rules! parse_forloop {
    () => {
        deps!();
        fn parse_forloop (pair : Pair < Rule >) -> TeraResult < Node > { let mut start_ws = WS :: default () ; let mut end_ws = WS :: default () ; let mut key = None ; let mut value = None ; let mut container = None ; let mut body = vec ! [] ; let mut empty_body : Option < Vec < Node > > = None ; for p in pair . into_inner () { match p . as_rule () { Rule :: for_tag => { let mut idents = vec ! [] ; for p2 in p . into_inner () { match p2 . as_rule () { Rule :: tag_start => start_ws . left = p2 . as_span () . as_str () == "{%-" , Rule :: tag_end => start_ws . right = p2 . as_span () . as_str () == "-%}" , Rule :: ident => idents . push (p2 . as_str () . to_string ()) , Rule :: basic_expr_filter => { container = Some (parse_basic_expr_with_filters (p2) ?) ; } Rule :: array_filter => container = Some (parse_array_with_filters (p2) ?) , _ => unreachable ! () , } ; } if idents . len () == 1 { value = Some (idents [0] . clone ()) ; } else { key = Some (idents [0] . clone ()) ; value = Some (idents [1] . clone ()) ; } } Rule :: content | Rule :: macro_content | Rule :: block_content | Rule :: filter_section_content | Rule :: for_content => { match empty_body { Some (ref mut empty_body) => empty_body . extend (parse_content (p) ?) , None => body . extend (parse_content (p) ?) , } ; } Rule :: else_tag => { empty_body = Some (vec ! []) ; } Rule :: endfor_tag => { for p2 in p . into_inner () { match p2 . as_rule () { Rule :: tag_start => end_ws . left = p2 . as_span () . as_str () == "{%-" , Rule :: tag_end => end_ws . right = p2 . as_span () . as_str () == "-%}" , Rule :: ident => () , _ => unreachable ! () , } ; } } _ => unreachable ! ("unexpected {:?} rule in parse_forloop" , p . as_rule ()) , } ; } Ok (Node :: Forloop (start_ws , Forloop { key , value : value . unwrap () , container : container . unwrap () , body , empty_body } , end_ws ,)) }
    };
}

parse_forloop!()