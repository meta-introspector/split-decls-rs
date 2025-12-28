macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'a > rustc_session :: HashStableContext for StableHashingContext < 'a > { }
    };
}

impl_121!()