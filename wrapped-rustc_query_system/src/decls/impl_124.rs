macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'ctx > rustc_ast :: HashStableContext for StableHashingContext < 'ctx > { }
    };
}

impl_124!()