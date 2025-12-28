macro_rules! deps {
    () => {
        Result!();
        Iter!();
        Expr!();
    };
}

macro_rules! parse_not {
    () => {
        deps!();
        fn parse_not (introducer : & Ident , iter : Iter) -> Result < Expr > { let paren = token :: parse_paren (introducer , iter) ? ; let ref mut inner = iter :: new (paren . stream ()) ; let expr = self :: parse (inner) ? ; token :: parse_optional_punct (inner , ',') ; token :: parse_end (inner) ? ; Ok (Expr :: Not (Box :: new (expr))) }
    };
}

parse_not!();