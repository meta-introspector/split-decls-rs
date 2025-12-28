macro_rules! deps {
    () => {
        Binder!();
        TypeVisitor!();
        TypeVisitable!();
        Interner!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Binder < I , T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { visitor . visit_binder (self) } }
    };
}

impl_223!();