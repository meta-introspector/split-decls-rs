macro_rules! deps {
    () => {
        Element!();
        SyntaxElement!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl Element for SyntaxElement { fn syntax_element (self) -> SyntaxElement { self } }
    };
}

impl_165!();