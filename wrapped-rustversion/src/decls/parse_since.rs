macro_rules! deps {
    () => {
        Expr!();
        Iter!();
        Result!();
    };
}

macro_rules! parse_since {
    () => {
        deps!();
        fn parse_since (introducer : & Ident , iter : Iter) -> Result < Expr > { let paren = token :: parse_paren (introducer , iter) ? ; let ref mut inner = iter :: new (paren . stream ()) ; let bound = bound :: parse (paren , inner) ? ; token :: parse_optional_punct (inner , ',') ; token :: parse_end (inner) ? ; Ok (Expr :: Since (bound)) }
    };
}

parse_since!();