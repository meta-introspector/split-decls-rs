macro_rules! deps {
    () => {
        Expr!();
        ExprVal!();
    };
}

macro_rules! parse_in_condition_container {
    () => {
        deps!();
        fn parse_in_condition_container (pair : Pair < Rule >) -> TeraResult < Expr > { let mut expr = None ; for p in pair . into_inner () { match p . as_rule () { Rule :: array_filter => expr = Some (parse_array_with_filters (p) ?) , Rule :: dotted_square_bracket_ident => { expr = Some (Expr :: new (ExprVal :: Ident (p . as_str () . to_string ()))) } Rule :: string_expr_filter => expr = Some (parse_string_expr_with_filters (p) ?) , _ => unreachable ! ("Got {:?} in parse_in_condition_container" , p) , } ; } Ok (expr . unwrap ()) }
    };
}

parse_in_condition_container!()