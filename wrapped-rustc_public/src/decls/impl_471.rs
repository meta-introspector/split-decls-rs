macro_rules! deps {
    () => {
        Visitable!();
        Visitor!();
    };
}

macro_rules! impl_471 {
    () => {
        deps!();
        impl < T : Visitable > Visitable for Vec < T > { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { for arg in self { arg . visit (visitor) ? ; } ControlFlow :: Continue (()) } }
    };
}

impl_471!()