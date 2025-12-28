macro_rules! deps {
    () => {
        TypeVisitable!();
        TypeVisitor!();
        Interner!();
    };
}

macro_rules! impl_491 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Vec < T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
    };
}

impl_491!();