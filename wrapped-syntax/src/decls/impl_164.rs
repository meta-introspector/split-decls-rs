macro_rules! deps {
    () => {
        Element!();
        SyntaxElement!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < E : Element + Clone > Element for & '_ E { fn syntax_element (self) -> SyntaxElement { self . clone () . syntax_element () } }
    };
}

impl_164!();