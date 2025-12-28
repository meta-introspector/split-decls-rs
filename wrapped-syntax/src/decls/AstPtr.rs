macro_rules! deps {
    () => {
        SyntaxNodePtr!();
        AstNode!();
    };
}

macro_rules! AstPtr {
    () => {
        deps!();
        # [doc = " Like `SyntaxNodePtr`, but remembers the type of node."] pub struct AstPtr < N : AstNode > { raw : SyntaxNodePtr , _ty : PhantomData < fn () -> N > , }
    };
}

AstPtr!()