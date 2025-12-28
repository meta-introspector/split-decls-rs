macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! parse_logic_val {
    () => {
        deps!();
        # [doc = " An expression that can be negated"] fn parse_logic_val (pair : Pair < Rule >) -> TeraResult < Expr > { let mut negated = false ; let mut expr = None ; for p in pair . into_inner () { match p . as_rule () { Rule :: op_not => negated = true , Rule :: in_cond => expr = Some (parse_in_condition (p) ?) , Rule :: comparison_expr => expr = Some (parse_comparison_expression (p) ?) , Rule :: string_expr_filter => expr = Some (parse_string_expr_with_filters (p) ?) , Rule :: logic_expr => expr = Some (parse_logic_expr (p) ?) , _ => unreachable ! () , } ; } let mut e = expr . unwrap () ; e . negated = negated ; Ok (e) }
    };
}

parse_logic_val!()