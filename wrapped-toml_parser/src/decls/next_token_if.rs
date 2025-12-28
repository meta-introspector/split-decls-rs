macro_rules! deps {
    () => {
        TokenKind!();
        Token!();
        Stream!();
    };
}

macro_rules! next_token_if {
    () => {
        deps!();
        fn next_token_if < 'i , F : Fn (TokenKind) -> bool > (tokens : & mut Stream < 'i > , pred : F ,) -> Option < & 'i Token > { match tokens . first () { Some (next) if pred (next . kind ()) => tokens . next_token () , _ => None , } }
    };
}

next_token_if!()