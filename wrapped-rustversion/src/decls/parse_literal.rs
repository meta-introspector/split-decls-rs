macro_rules! deps {
    () => {
        Result!();
        Iter!();
        Error!();
    };
}

macro_rules! parse_literal {
    () => {
        deps!();
        pub fn parse_literal (iter : Iter) -> Result < Literal > { match iter . next () { Some (TokenTree :: Literal (literal)) => Ok (literal) , unexpected => { let span = unexpected . as_ref () . map_or_else (Span :: call_site , TokenTree :: span) ; Err (Error :: new (span , "expected literal")) } } }
    };
}

parse_literal!();