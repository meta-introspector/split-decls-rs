macro_rules! deps {
    () => {
        SyntaxElement!();
        Element!();
        SyntaxToken!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl Element for SyntaxToken { fn syntax_element (self) -> SyntaxElement { self . into () } }
    };
}

impl_167!()