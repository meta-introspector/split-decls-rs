macro_rules! deps {
    () => {
        SyntaxNode!();
        Element!();
        SyntaxElement!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl Element for SyntaxNode { fn syntax_element (self) -> SyntaxElement { self . into () } }
    };
}

impl_157!()