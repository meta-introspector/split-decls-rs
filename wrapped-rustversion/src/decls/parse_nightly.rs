macro_rules! deps {
    () => {
        Expr!();
        Date!();
        Iter!();
        Result!();
    };
}

macro_rules! parse_nightly {
    () => {
        deps!();
        fn parse_nightly (iter : Iter) -> Result < Expr > { let paren = match token :: parse_optional_paren (iter) { Some (group) => group , None => return Ok (Expr :: Nightly) , } ; let ref mut inner = iter :: new (paren . stream ()) ; let date = date :: parse (paren , inner) ? ; token :: parse_optional_punct (inner , ',') ; token :: parse_end (inner) ? ; Ok (Expr :: Date (date)) }
    };
}

parse_nightly!();