macro_rules! deps {
    () => {
        NormalizeToExpected!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl Default for NormalizeToExpected < '_ > { fn default () -> Self { Self :: new () } }
    };
}

impl_205!()