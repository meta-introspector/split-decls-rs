macro_rules! deps {
    () => {
        Visitable!();
        ExistentialPredicate!();
        Visitor!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl Visitable for ExistentialPredicate { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match self { ExistentialPredicate :: Trait (tr) => tr . generic_args . visit (visitor) , ExistentialPredicate :: Projection (p) => { p . term . visit (visitor) ? ; p . generic_args . visit (visitor) } ExistentialPredicate :: AutoTrait (_) => ControlFlow :: Continue (()) , } } }
    };
}

impl_473!();