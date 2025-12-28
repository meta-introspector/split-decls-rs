macro_rules! deps {
    () => {
        Interner!();
        IndexSet!();
        TypeVisitor!();
        TypeVisitable!();
    };
}

macro_rules! impl_497 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > , S > TypeVisitable < I > for indexmap :: IndexSet < T , S > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
    };
}

impl_497!()