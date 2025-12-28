macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl Default for Service < '_ > { fn default () -> Self { Self :: new () } }
    };
}

impl_38!();