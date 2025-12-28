macro_rules! deps {
    () => {
        HasDepContext!();
        DepContext!();
        Deps!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < T : DepContext > HasDepContext for T { type Deps = T :: Deps ; type DepContext = Self ; fn dep_context (& self) -> & Self :: DepContext { self } }
    };
}

impl_101!()