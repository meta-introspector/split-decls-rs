macro_rules! deps {
    () => {
        DepContext!();
        HasDepContext!();
        Deps!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < T : HasDepContext , Q : Copy > HasDepContext for (T , Q) { type Deps = T :: Deps ; type DepContext = T :: DepContext ; fn dep_context (& self) -> & Self :: DepContext { self . 0 . dep_context () } }
    };
}

impl_102!()