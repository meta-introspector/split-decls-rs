macro_rules! deps {
    () => {
        TokenKind!();
        Span!();
        Token!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl Token { pub (super) fn new (kind : TokenKind , span : Span) -> Self { Self { kind , span } } # [inline (always)] pub fn kind (& self) -> TokenKind { self . kind } # [inline (always)] pub fn span (& self) -> Span { self . span } }
    };
}

impl_122!();