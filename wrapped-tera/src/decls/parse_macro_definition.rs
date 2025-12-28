macro_rules! deps {
    () => {
        Node!();
        WS!();
        MacroDefinition!();
    };
}

macro_rules! parse_macro_definition {
    () => {
        deps!();
        fn parse_macro_definition (pair : Pair < Rule >) -> TeraResult < Node > { let mut start_ws = WS :: default () ; let mut end_ws = WS :: default () ; let mut name = String :: new () ; let mut args = HashMap :: new () ; let mut body = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: macro_tag => { for p2 in p . into_inner () { match p2 . as_rule () { Rule :: tag_start => start_ws . left = p2 . as_span () . as_str () == "{%-" , Rule :: tag_end => start_ws . right = p2 . as_span () . as_str () == "-%}" , Rule :: macro_fn_wrapper => { let macro_fn = parse_macro_fn (p2) ? ; name = macro_fn . 0 ; args = macro_fn . 1 ; } _ => continue , } ; } } Rule :: macro_content => body . extend (parse_content (p) ?) , Rule :: endmacro_tag => { for p2 in p . into_inner () { match p2 . as_rule () { Rule :: tag_start => end_ws . left = p2 . as_span () . as_str () == "{%-" , Rule :: tag_end => end_ws . right = p2 . as_span () . as_str () == "-%}" , Rule :: ident => () , _ => unreachable ! () , } ; } } _ => unreachable ! ("unexpected {:?} rule in parse_macro_definition" , p . as_rule ()) , } } Ok (Node :: MacroDefinition (start_ws , MacroDefinition { name , args , body } , end_ws)) }
    };
}

parse_macro_definition!();