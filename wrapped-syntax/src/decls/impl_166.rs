macro_rules! deps {
    () => {
        SyntaxNode!();
        Element!();
        SyntaxElement!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl Element for SyntaxNode { fn syntax_element (self) -> SyntaxElement { self . into () } }
    };
}

impl_166!()