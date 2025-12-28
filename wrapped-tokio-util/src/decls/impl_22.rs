macro_rules! deps {
    () => {
        JoinMap!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < K , V > Default for JoinMap < K , V > { fn default () -> Self { Self :: new () } }
    };
}

impl_22!();