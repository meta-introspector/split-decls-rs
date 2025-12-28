macro_rules! deps {
    () => {
        LocalDefId!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl ! PartialOrd for LocalDefId { }
    };
}

impl_115!();