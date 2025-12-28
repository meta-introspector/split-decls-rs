macro_rules! deps {
    () => {
        Token!();
        Lexer!();
        TokenKind!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 's > Lexer < 's > { fn new (input : & 's str) -> Self { Self { stream : input } } fn unknown (& mut self) -> Option < Token < 's > > { let remaining = self . stream . len () ; if remaining == 0 { return None ; } let raw = self . stream ; self . stream = & self . stream [remaining .. remaining] ; Some (Token { kind : TokenKind :: Unknown , raw , }) } }
    };
}

impl_23!();