macro_rules! deps {
    () => {
        Interner!();
        TypeVisitor!();
        TypeVisitable!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Option < T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { match self { Some (v) => v . visit_with (visitor) , None => V :: Result :: output () , } } }
    };
}

impl_487!();