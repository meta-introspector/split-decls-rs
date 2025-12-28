macro_rules! deps {
    () => {
        Result!();
        Expr!();
        Iter!();
    };
}

macro_rules! parse_all {
    () => {
        deps!();
        fn parse_all (introducer : & Ident , iter : Iter) -> Result < Expr > { let paren = token :: parse_paren (introducer , iter) ? ; let ref mut inner = iter :: new (paren . stream ()) ; let exprs = parse_comma_separated (inner) ? ; Ok (Expr :: All (exprs . into_iter () . collect ())) }
    };
}

parse_all!();