macro_rules! deps {
    () => {
        TypeVisitable!();
        TypeVisitor!();
        Interner!();
    };
}

macro_rules! impl_486 {
    () => {
        deps!();
        impl < I : Interner , A : TypeVisitable < I > , B : TypeVisitable < I > , C : TypeVisitable < I > > TypeVisitable < I > for (A , B , C) { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { try_visit ! (self . 0 . visit_with (visitor)) ; try_visit ! (self . 1 . visit_with (visitor)) ; self . 2 . visit_with (visitor) } }
    };
}

impl_486!()