macro_rules! deps {
    () => {
        TypeVisitor!();
        TypeVisitable!();
        Interner!();
    };
}

macro_rules! impl_494 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for & [T] { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
    };
}

impl_494!()