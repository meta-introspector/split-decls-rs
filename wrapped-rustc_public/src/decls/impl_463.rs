macro_rules! deps {
    () => {
        Visitor!();
        UnevaluatedConst!();
        Visitable!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl Visitable for UnevaluatedConst { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { let UnevaluatedConst { def , args , promoted } = self ; def . visit (visitor) ? ; args . visit (visitor) ? ; promoted . visit (visitor) } }
    };
}

impl_463!()