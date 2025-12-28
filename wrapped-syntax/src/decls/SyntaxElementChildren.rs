macro_rules! deps {
    () => {
        RustLanguage!();
    };
}

macro_rules! SyntaxElementChildren {
    () => {
        deps!();
        pub type SyntaxElementChildren = rowan :: SyntaxElementChildren < RustLanguage > ;
    };
}

SyntaxElementChildren!();