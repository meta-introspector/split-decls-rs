macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < K , V > Default for Map < K , V > where K : Ord + Hash , { # [inline] fn default () -> Self { Self :: new () } }
    };
}

impl_9!()