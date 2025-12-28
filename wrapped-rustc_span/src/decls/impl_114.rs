macro_rules! deps {
    () => {
        LocalDefId!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl ! Ord for LocalDefId { }
    };
}

impl_114!()