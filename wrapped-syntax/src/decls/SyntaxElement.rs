macro_rules! deps {
    () => {
        RustLanguage!();
    };
}

macro_rules! SyntaxElement {
    () => {
        deps!();
        pub type SyntaxElement = rowan :: SyntaxElement < RustLanguage > ;
    };
}

SyntaxElement!();