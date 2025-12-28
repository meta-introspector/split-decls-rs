macro_rules! deps {
    () => {
        SyntaxToken!();
        SyntaxElement!();
        Element!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl Element for SyntaxToken { fn syntax_element (self) -> SyntaxElement { self . into () } }
    };
}

impl_158!()