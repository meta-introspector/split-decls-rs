macro_rules! deps {
    () => {
        SyntaxNode!();
        SyntaxNodeChildren!();
    };
}

macro_rules! AstChildren {
    () => {
        deps!();
        # [doc = " An iterator over `SyntaxNode` children of a particular AST type."] # [derive (Debug , Clone)] pub struct AstChildren < N > { inner : SyntaxNodeChildren , ph : PhantomData < N > , }
    };
}

AstChildren!()