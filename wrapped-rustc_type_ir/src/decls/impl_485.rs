macro_rules! deps {
    () => {
        TypeVisitor!();
        Interner!();
        TypeVisitable!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > , U : TypeVisitable < I > > TypeVisitable < I > for (T , U) { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { try_visit ! (self . 0 . visit_with (visitor)) ; self . 1 . visit_with (visitor) } }
    };
}

impl_485!()