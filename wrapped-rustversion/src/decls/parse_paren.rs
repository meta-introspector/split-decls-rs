macro_rules! deps {
    () => {
        Error!();
        Iter!();
        Result!();
    };
}

macro_rules! parse_paren {
    () => {
        deps!();
        pub fn parse_paren (introducer : & Ident , iter : Iter) -> Result < Group > { match iter . peek () { Some (TokenTree :: Group (group)) if group . delimiter () == Delimiter :: Parenthesis => { match iter . next () { Some (TokenTree :: Group (group)) => Ok (group) , _ => unreachable ! () , } } Some (unexpected) => Err (Error :: new (unexpected . span () , "expected `(`")) , None => Err (Error :: new (introducer . span () , format ! ("expected `(` after `{}`" , introducer) ,)) , } }
    };
}

parse_paren!()