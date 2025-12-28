macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! parse_optional_punct {
    () => {
        deps!();
        pub fn parse_optional_punct (iter : Iter , ch : char) -> Option < () > { match iter . peek () { Some (TokenTree :: Punct (punct)) if punct . as_char () == ch => iter . next () . map (drop) , _ => None , } }
    };
}

parse_optional_punct!();