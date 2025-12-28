macro_rules! deps {
    () => {
        Ty!();
        MirConst!();
        Visitor!();
        ConstantKind!();
        Visitable!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl Visitable for MirConst { fn visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { self . super_visit (visitor) } fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match & self . kind () { super :: ty :: ConstantKind :: Ty (ct) => ct . visit (visitor) ? , super :: ty :: ConstantKind :: Allocated (alloc) => alloc . visit (visitor) ? , super :: ty :: ConstantKind :: Unevaluated (uv) => uv . visit (visitor) ? , super :: ty :: ConstantKind :: Param (_) | super :: ty :: ConstantKind :: ZeroSized => { } } self . ty () . visit (visitor) } }
    };
}

impl_460!()