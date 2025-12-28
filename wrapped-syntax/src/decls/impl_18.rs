macro_rules! deps {
    () => {
        Parse!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Parse < SyntaxNode > { pub fn cast < N : AstNode > (mut self) -> Option < Parse < N > > { if N :: cast (self . syntax_node ()) . is_some () { Some (Parse { green : self . green . take () , errors : self . errors . take () , _ty : PhantomData }) } else { None } } }
    };
}

impl_18!()