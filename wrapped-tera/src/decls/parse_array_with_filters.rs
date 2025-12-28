macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! parse_array_with_filters {
    () => {
        deps!();
        # [doc = " An array with optional filters"] fn parse_array_with_filters (pair : Pair < Rule >) -> TeraResult < Expr > { let mut array = None ; let mut filters = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: array => array = Some (parse_array (p) ?) , Rule :: filter => filters . push (parse_filter (p) ?) , _ => unreachable ! ("Got {:?}" , p) , } ; } Ok (Expr { val : array . unwrap () , negated : false , filters }) }
    };
}

parse_array_with_filters!()