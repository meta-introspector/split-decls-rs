macro_rules! deps {
    () => {
        Token!();
        Event!();
        Span!();
    };
}

macro_rules! sealed {
    () => {
        deps!();
        mod sealed { pub trait Sealed { } impl Sealed for crate :: Span { } impl Sealed for & crate :: Span { } impl Sealed for crate :: lexer :: Token { } impl Sealed for & crate :: lexer :: Token { } impl Sealed for crate :: parser :: Event { } impl Sealed for & crate :: parser :: Event { } }
    };
}

sealed!()