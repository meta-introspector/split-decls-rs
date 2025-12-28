macro_rules! deps {
    () => {
        GenericArgs!();
        Visitor!();
        Visitable!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl Visitable for GenericArgs { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { self . 0 . visit (visitor) } }
    };
}

impl_467!()