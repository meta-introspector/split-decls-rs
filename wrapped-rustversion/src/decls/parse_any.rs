macro_rules! deps {
    () => {
        Iter!();
        Expr!();
        Result!();
    };
}

macro_rules! parse_any {
    () => {
        deps!();
        fn parse_any (introducer : & Ident , iter : Iter) -> Result < Expr > { let paren = token :: parse_paren (introducer , iter) ? ; let ref mut inner = iter :: new (paren . stream ()) ; let exprs = parse_comma_separated (inner) ? ; Ok (Expr :: Any (exprs . into_iter () . collect ())) }
    };
}

parse_any!();