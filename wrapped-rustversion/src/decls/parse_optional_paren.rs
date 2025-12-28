macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! parse_optional_paren {
    () => {
        deps!();
        pub fn parse_optional_paren (iter : Iter) -> Option < Group > { match iter . peek () { Some (TokenTree :: Group (group)) if group . delimiter () == Delimiter :: Parenthesis => { match iter . next () { Some (TokenTree :: Group (group)) => Some (group) , _ => unreachable ! () , } } _ => None , } }
    };
}

parse_optional_paren!();