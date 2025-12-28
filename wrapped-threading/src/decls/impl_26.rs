macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Default for Pool { fn default () -> Self { Self :: new () } }
    };
}

impl_26!()