macro_rules! deps {
    () => {
        SsoHashMap!();
    };
}

macro_rules! impl_443 {
    () => {
        deps!();
        impl < K , V > Default for SsoHashMap < K , V > { # [inline] fn default () -> Self { Self :: new () } }
    };
}

impl_443!();