macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! err_unexpected_token {
    () => {
        deps!();
        fn err_unexpected_token (span : Span , delimiter : Delimiter) -> Error { let msg = match delimiter { Delimiter :: Parenthesis => "unexpected token, expected `)`" , Delimiter :: Brace => "unexpected token, expected `}`" , Delimiter :: Bracket => "unexpected token, expected `]`" , Delimiter :: None => "unexpected token" , } ; Error :: new (span , msg) }
    };
}

err_unexpected_token!()