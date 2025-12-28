macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl Default for Value { fn default () -> Self { Self :: Null } }
    };
}

impl_121!();