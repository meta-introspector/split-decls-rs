macro_rules! deps {
    () => {
        Visitable!();
        Visitor!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        impl Visitable for ConstDef { fn super_visit < V : Visitor > (& self , _visitor : & mut V) -> ControlFlow < V :: Break > { ControlFlow :: Continue (()) } }
    };
}

impl_464!()