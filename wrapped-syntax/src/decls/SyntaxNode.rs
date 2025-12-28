macro_rules! deps {
    () => {
        RustLanguage!();
    };
}

macro_rules! SyntaxNode {
    () => {
        deps!();
        pub type SyntaxNode = rowan :: SyntaxNode < RustLanguage > ;
    };
}

SyntaxNode!()