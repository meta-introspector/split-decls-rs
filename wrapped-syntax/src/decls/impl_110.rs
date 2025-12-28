macro_rules! deps {
    () => {
        SyntaxNode!();
        AstChildren!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < N > AstChildren < N > { fn new (parent : & SyntaxNode) -> Self { AstChildren { inner : parent . children () , ph : PhantomData } } }
    };
}

impl_110!()