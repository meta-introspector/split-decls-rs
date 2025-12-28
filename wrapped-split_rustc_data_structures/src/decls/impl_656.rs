macro_rules! deps {
    () => {
        UnordCollection!();
        UnordMap!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        impl < K : Eq + Hash , V > UnordCollection for UnordMap < K , V > { }
    };
}

impl_656!();