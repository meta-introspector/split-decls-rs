macro_rules! deps {
    () => {
        Test!();
    };
}

macro_rules! parse_test {
    () => {
        deps!();
        fn parse_test (pair : Pair < Rule >) -> TeraResult < Test > { let mut ident = None ; let mut name = None ; let mut args = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: dotted_square_bracket_ident => ident = Some (p . as_str () . to_string ()) , Rule :: test_call => { let (_name , _args) = parse_test_call (p) ? ; name = Some (_name) ; args = _args ; } _ => unreachable ! ("{:?} not supposed to get there (parse_ident)!" , p . as_rule ()) , } ; } Ok (Test { ident : ident . unwrap () , negated : false , name : name . unwrap () , args }) }
    };
}

parse_test!();