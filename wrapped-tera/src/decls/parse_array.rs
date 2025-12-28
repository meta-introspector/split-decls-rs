macro_rules! deps {
    () => {
        ExprVal!();
    };
}

macro_rules! parse_array {
    () => {
        deps!();
        fn parse_array (pair : Pair < Rule >) -> TeraResult < ExprVal > { let mut vals = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: logic_val => { vals . push (parse_logic_val (p) ?) ; } _ => unreachable ! ("Got {:?} in parse_array" , p . as_rule ()) , } } Ok (ExprVal :: Array (vals)) }
    };
}

parse_array!();