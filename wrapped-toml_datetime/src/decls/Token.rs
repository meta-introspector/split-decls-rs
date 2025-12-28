macro_rules! deps {
    () => {
        TokenKind!();
    };
}

macro_rules! Token {
    () => {
        deps!();
        # [derive (Copy , Clone)] struct Token < 's > { kind : TokenKind , raw : & 's str , }
    };
}

Token!();