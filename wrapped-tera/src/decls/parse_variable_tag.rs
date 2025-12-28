macro_rules! deps {
    () => {
        WS!();
        Node!();
    };
}

macro_rules! parse_variable_tag {
    () => {
        deps!();
        fn parse_variable_tag (pair : Pair < Rule >) -> TeraResult < Node > { let mut ws = WS :: default () ; let mut expr = None ; for p in pair . into_inner () { match p . as_rule () { Rule :: variable_start => { ws . left = p . as_span () . as_str () == "{{-" ; } Rule :: variable_end => { ws . right = p . as_span () . as_str () == "-}}" ; } Rule :: logic_expr => expr = Some (parse_logic_expr (p) ?) , Rule :: array_filter => expr = Some (parse_array_with_filters (p) ?) , _ => unreachable ! ("unexpected {:?} rule in parse_variable_tag" , p . as_rule ()) , } } Ok (Node :: VariableBlock (ws , expr . unwrap ())) }
    };
}

parse_variable_tag!();