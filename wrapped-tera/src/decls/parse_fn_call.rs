macro_rules! deps {
    () => {
        FunctionCall!();
    };
}

macro_rules! parse_fn_call {
    () => {
        deps!();
        fn parse_fn_call (pair : Pair < Rule >) -> TeraResult < FunctionCall > { let mut name = None ; let mut args = HashMap :: new () ; for p in pair . into_inner () { match p . as_rule () { Rule :: ident => name = Some (p . as_span () . as_str () . to_string ()) , Rule :: kwarg => { let (name , val) = parse_kwarg (p) ? ; args . insert (name , val) ; } _ => unreachable ! ("{:?} not supposed to get there (parse_fn_call)!" , p . as_rule ()) , } ; } Ok (FunctionCall { name : name . unwrap () , args }) }
    };
}

parse_fn_call!();