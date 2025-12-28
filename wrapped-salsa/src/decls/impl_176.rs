macro_rules! deps {
    () => {
        InternedData!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < T : Eq + Hash + Clone + Sync + Send > InternedData for T { }
    };
}

impl_176!();