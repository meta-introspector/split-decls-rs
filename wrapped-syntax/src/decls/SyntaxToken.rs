macro_rules! deps {
    () => {
        RustLanguage!();
    };
}

macro_rules! SyntaxToken {
    () => {
        deps!();
        pub type SyntaxToken = rowan :: SyntaxToken < RustLanguage > ;
    };
}

SyntaxToken!()