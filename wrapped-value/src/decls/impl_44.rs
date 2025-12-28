macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl Default for Value { fn default () -> Self { Self :: Null } }
    };
}

impl_44!()