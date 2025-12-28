macro_rules! deps {
    () => {
        TypeVisitor!();
        TypeVisitable!();
        Interner!();
    };
}

macro_rules! impl_489 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Arc < T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { (* * self) . visit_with (visitor) } }
    };
}

impl_489!();