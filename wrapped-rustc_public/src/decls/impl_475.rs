macro_rules! deps {
    () => {
        Visitable!();
        FnSig!();
        Visitor!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl Visitable for FnSig { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { self . inputs_and_output . visit (visitor) } }
    };
}

impl_475!();