macro_rules! deps {
    () => {
        DefaultCache!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < K , V > Default for DefaultCache < K , V > { fn default () -> Self { DefaultCache { cache : Default :: default () } } }
    };
}

impl_191!()