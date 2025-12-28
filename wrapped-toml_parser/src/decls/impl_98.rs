macro_rules! deps {
    () => {
        TokenKind!();
        Encoding!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl Encoding { pub const fn description (& self) -> & 'static str { match self { Self :: LiteralString => crate :: lexer :: TokenKind :: LiteralString . description () , Self :: BasicString => crate :: lexer :: TokenKind :: BasicString . description () , Self :: MlLiteralString => crate :: lexer :: TokenKind :: MlLiteralString . description () , Self :: MlBasicString => crate :: lexer :: TokenKind :: MlBasicString . description () , } } }
    };
}

impl_98!();