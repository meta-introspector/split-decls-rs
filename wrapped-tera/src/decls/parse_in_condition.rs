macro_rules! deps {
    () => {
        Expr!();
        In!();
        ExprVal!();
    };
}

macro_rules! parse_in_condition {
    () => {
        deps!();
        fn parse_in_condition (pair : Pair < Rule >) -> TeraResult < Expr > { let mut lhs = None ; let mut rhs = None ; let mut negated = false ; for p in pair . into_inner () { match p . as_rule () { Rule :: string_expr_filter => lhs = Some (parse_string_expr_with_filters (p) ?) , Rule :: basic_expr_filter => lhs = Some (parse_basic_expr_with_filters (p) ?) , Rule :: in_cond_container => rhs = Some (parse_in_condition_container (p) ?) , Rule :: op_not => negated = true , _ => unreachable ! ("Got {:?} in parse_in_condition" , p) , } ; } Ok (Expr :: new (ExprVal :: In (In { lhs : Box :: new (lhs . unwrap ()) , rhs : Box :: new (rhs . unwrap ()) , negated , }))) }
    };
}

parse_in_condition!()