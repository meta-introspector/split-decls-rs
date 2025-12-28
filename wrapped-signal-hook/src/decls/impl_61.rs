macro_rules! deps {
    () => {
        Channel!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T > Default for Channel < T > { fn default () -> Self { Self :: new () } }
    };
}

impl_61!();