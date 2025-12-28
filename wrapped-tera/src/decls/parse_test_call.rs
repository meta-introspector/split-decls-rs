macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! parse_test_call {
    () => {
        deps!();
        fn parse_test_call (pair : Pair < Rule >) -> TeraResult < (String , Vec < Expr >) > { let mut name = None ; let mut args = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: ident => name = Some (p . as_span () . as_str () . to_string ()) , Rule :: test_arg => { for p2 in p . into_inner () { match p2 . as_rule () { Rule :: logic_expr => { args . push (parse_logic_expr (p2) ?) ; } Rule :: array => { args . push (Expr :: new (parse_array (p2) ?)) ; } _ => unreachable ! ("Invalid arg type for test {:?}" , p2 . as_rule ()) , } } } _ => unreachable ! ("{:?} not supposed to get there (parse_test_call)!" , p . as_rule ()) , } ; } Ok ((name . unwrap () , args)) }
    };
}

parse_test_call!()