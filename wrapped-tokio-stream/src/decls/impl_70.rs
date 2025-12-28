macro_rules! deps {
    () => {
        StreamMap!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < K , V > Default for StreamMap < K , V > { fn default () -> Self { Self :: new () } }
    };
}

impl_70!();