macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! parse_optional_keyword {
    () => {
        deps!();
        pub fn parse_optional_keyword (iter : Iter , keyword : & str) -> Option < Span > { match iter . peek () { Some (TokenTree :: Ident (ident)) if ident . to_string () == keyword => { Some (iter . next () . unwrap () . span ()) } _ => None , } }
    };
}

parse_optional_keyword!();