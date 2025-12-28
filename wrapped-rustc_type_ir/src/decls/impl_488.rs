macro_rules! deps {
    () => {
        TypeVisitor!();
        Interner!();
        TypeVisitable!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > , E : TypeVisitable < I > > TypeVisitable < I > for Result < T , E > { fn visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result { match self { Ok (v) => v . visit_with (visitor) , Err (e) => e . visit_with (visitor) , } } }
    };
}

impl_488!();