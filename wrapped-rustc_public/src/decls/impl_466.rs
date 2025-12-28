macro_rules! deps {
    () => {
        Promoted!();
        Visitor!();
        Visitable!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        impl Visitable for Promoted { fn super_visit < V : Visitor > (& self , _visitor : & mut V) -> ControlFlow < V :: Break > { ControlFlow :: Continue (()) } }
    };
}

impl_466!()