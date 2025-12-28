macro_rules! deps {
    () => {
        Visitable!();
        Region!();
        Visitor!();
    };
}

macro_rules! impl_468 {
    () => {
        deps!();
        impl Visitable for Region { fn visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { visitor . visit_reg (self) } fn super_visit < V : Visitor > (& self , _ : & mut V) -> ControlFlow < V :: Break > { ControlFlow :: Continue (()) } }
    };
}

impl_468!();