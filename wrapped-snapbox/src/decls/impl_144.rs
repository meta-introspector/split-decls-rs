macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl Default for Data { fn default () -> Self { Self :: new () } }
    };
}

impl_144!()