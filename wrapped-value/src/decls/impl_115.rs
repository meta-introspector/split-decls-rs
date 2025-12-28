macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl Default for ConstValue { fn default () -> Self { Self :: Null } }
    };
}

impl_115!()