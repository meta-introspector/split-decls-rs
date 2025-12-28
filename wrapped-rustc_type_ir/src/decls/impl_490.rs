macro_rules! deps {
    () => {
        Interner!();
        TypeVisitable!();
        TypeVisitor!();
    };
}

macro_rules! impl_490 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > > TypeVisitable < I > for Box < T > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { (* * self) . visit_with (visitor) } }
    };
}

impl_490!()