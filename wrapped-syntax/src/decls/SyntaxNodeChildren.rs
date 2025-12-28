macro_rules! deps {
    () => {
        RustLanguage!();
    };
}

macro_rules! SyntaxNodeChildren {
    () => {
        deps!();
        pub type SyntaxNodeChildren = rowan :: SyntaxNodeChildren < RustLanguage > ;
    };
}

SyntaxNodeChildren!()