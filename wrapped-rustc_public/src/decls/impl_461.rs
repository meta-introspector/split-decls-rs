macro_rules! deps {
    () => {
        Visitable!();
        Visitor!();
        Opaque!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl Visitable for Opaque { fn super_visit < V : Visitor > (& self , _visitor : & mut V) -> ControlFlow < V :: Break > { ControlFlow :: Continue (()) } }
    };
}

impl_461!()