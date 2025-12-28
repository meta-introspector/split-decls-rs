macro_rules! deps {
    () => {
        Span!();
        Token!();
        TokenKind!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl Token { pub (super) fn new (kind : TokenKind , span : Span) -> Self { Self { kind , span } } # [inline (always)] pub fn kind (& self) -> TokenKind { self . kind } # [inline (always)] pub fn span (& self) -> Span { self . span } }
    };
}

impl_122!()