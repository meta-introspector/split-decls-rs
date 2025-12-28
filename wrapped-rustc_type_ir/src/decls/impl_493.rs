macro_rules! deps {
    () => {
        Interner!();
        TypeVisitable!();
        TypeVisitor!();
    };
}

macro_rules! impl_493 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > , const N : usize > TypeVisitable < I > for SmallVec < [T ; N] > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
    };
}

impl_493!()