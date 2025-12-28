macro_rules! deps {
    () => {
        Visitable!();
        GenericArgKind!();
        Visitor!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl Visitable for GenericArgKind { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match self { GenericArgKind :: Lifetime (lt) => lt . visit (visitor) , GenericArgKind :: Type (t) => t . visit (visitor) , GenericArgKind :: Const (c) => c . visit (visitor) , } } }
    };
}

impl_469!()