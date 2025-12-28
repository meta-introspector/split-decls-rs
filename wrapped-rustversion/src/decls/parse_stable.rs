macro_rules! deps {
    () => {
        Expr!();
        Result!();
        Iter!();
        Release!();
    };
}

macro_rules! parse_stable {
    () => {
        deps!();
        fn parse_stable (iter : Iter) -> Result < Expr > { let paren = match token :: parse_optional_paren (iter) { Some (group) => group , None => return Ok (Expr :: Stable) , } ; let ref mut inner = iter :: new (paren . stream ()) ; let release = release :: parse (paren , inner) ? ; token :: parse_optional_punct (inner , ',') ; token :: parse_end (inner) ? ; Ok (Expr :: Release (release)) }
    };
}

parse_stable!()