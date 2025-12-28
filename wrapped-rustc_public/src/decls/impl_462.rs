macro_rules! deps {
    () => {
        Visitor!();
        Allocation!();
        Visitable!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl Visitable for Allocation { fn super_visit < V : Visitor > (& self , _visitor : & mut V) -> ControlFlow < V :: Break > { ControlFlow :: Continue (()) } }
    };
}

impl_462!();