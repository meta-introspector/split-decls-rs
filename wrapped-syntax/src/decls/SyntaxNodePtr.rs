macro_rules! deps {
    () => {
        SyntaxNode!();
        RustLanguage!();
    };
}

macro_rules! SyntaxNodePtr {
    () => {
        deps!();
        # [doc = " A \"pointer\" to a [`SyntaxNode`], via location in the source code."] pub type SyntaxNodePtr = rowan :: ast :: SyntaxNodePtr < RustLanguage > ;
    };
}

SyntaxNodePtr!()