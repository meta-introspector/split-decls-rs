macro_rules! deps {
    () => {
        Result!();
        Expr!();
        Iter!();
    };
}

macro_rules! parse_comma_separated {
    () => {
        deps!();
        fn parse_comma_separated (iter : Iter) -> Result < Vec < Expr > > { let mut exprs = Vec :: new () ; while iter . peek () . is_some () { let expr = self :: parse (iter) ? ; exprs . push (expr) ; if iter . peek () . is_none () { break ; } token :: parse_punct (iter , ',') ? ; } Ok (exprs) }
    };
}

parse_comma_separated!();