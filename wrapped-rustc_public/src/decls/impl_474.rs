macro_rules! deps {
    () => {
        Visitor!();
        Visitable!();
        TermKind!();
    };
}

macro_rules! impl_474 {
    () => {
        deps!();
        impl Visitable for TermKind { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match self { TermKind :: Type (t) => t . visit (visitor) , TermKind :: Const (c) => c . visit (visitor) , } } }
    };
}

impl_474!()