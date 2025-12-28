macro_rules! deps {
    () => {
        Visitor!();
        Visitable!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl < T : Visitable > Visitable for Option < T > { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match self { Some (val) => val . visit (visitor) , None => ControlFlow :: Continue (()) , } } }
    };
}

impl_465!();