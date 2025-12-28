macro_rules! deps {
    () => {
        DelayedMap!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < K , V > Default for DelayedMap < K , V > { fn default () -> Self { DelayedMap { cache : Default :: default () , count : 0 } } }
    };
}

impl_5!()