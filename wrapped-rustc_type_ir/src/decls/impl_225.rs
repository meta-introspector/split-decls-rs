macro_rules! deps {
    () => {
        Binder!();
        TypeVisitable!();
        Interner!();
        TypeSuperVisitable!();
        TypeVisitor!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > > TypeSuperVisitable < I > for Binder < I , T > { fn super_visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { self . as_ref () . skip_binder () . visit_with (visitor) } }
    };
}

impl_225!();