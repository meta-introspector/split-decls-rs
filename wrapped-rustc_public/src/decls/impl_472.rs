macro_rules! deps {
    () => {
        Visitor!();
        Visitable!();
        Binder!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < T : Visitable > Visitable for Binder < T > { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { self . value . visit (visitor) } }
    };
}

impl_472!()