macro_rules! deps {
    () => {
        AstNode!();
        AstPtr!();
        SyntaxNodePtr!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < N : AstNode > From < AstPtr < N > > for SyntaxNodePtr { fn from (ptr : AstPtr < N >) -> SyntaxNodePtr { ptr . raw } }
    };
}

impl_14!();