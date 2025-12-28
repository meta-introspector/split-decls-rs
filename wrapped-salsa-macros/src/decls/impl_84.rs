macro_rules! deps {
    () => {
        ToDbLifetimeVisitor!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl syn :: visit_mut :: VisitMut for ToDbLifetimeVisitor { fn visit_lifetime_mut (& mut self , i : & mut syn :: Lifetime) { i . clone_from (& self . db_lifetime) ; } }
    };
}

impl_84!()