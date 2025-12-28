macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Default for Service < '_ > { fn default () -> Self { Self :: new () } }
    };
}

impl_7!()