macro_rules! deps {
    () => {
        TypeVisitor!();
        Interner!();
        TypeVisitable!();
    };
}

macro_rules! impl_496 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > , Ix : Idx > TypeVisitable < I > for IndexVec < Ix , T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { walk_visitable_list ! (visitor , self . iter ()) ; V :: Result :: output () } }
    };
}

impl_496!();