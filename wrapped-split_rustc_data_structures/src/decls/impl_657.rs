macro_rules! deps {
    () => {
        UnordMap!();
    };
}

macro_rules! impl_657 {
    () => {
        deps!();
        impl < K : Eq + Hash , V > Default for UnordMap < K , V > { # [inline] fn default () -> Self { Self { inner : FxHashMap :: default () } } }
    };
}

impl_657!()