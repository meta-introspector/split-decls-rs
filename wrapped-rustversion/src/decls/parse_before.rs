macro_rules! deps {
    () => {
        Expr!();
        Result!();
        Iter!();
    };
}

macro_rules! parse_before {
    () => {
        deps!();
        fn parse_before (introducer : & Ident , iter : Iter) -> Result < Expr > { let paren = token :: parse_paren (introducer , iter) ? ; let ref mut inner = iter :: new (paren . stream ()) ; let bound = bound :: parse (paren , inner) ? ; token :: parse_optional_punct (inner , ',') ; token :: parse_end (inner) ? ; Ok (Expr :: Before (bound)) }
    };
}

parse_before!()