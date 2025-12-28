macro_rules! deps {
    () => {
        Interner!();
        TypeVisitable!();
        TypeVisitor!();
    };
}

macro_rules! impl_495 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Box < [T] > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
    };
}

impl_495!();