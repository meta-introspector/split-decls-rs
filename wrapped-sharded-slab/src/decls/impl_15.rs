macro_rules! deps {
    () => {
        Pool!();
        Clear!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T > Default for Pool < T > where T : Clear + Default , { fn default () -> Self { Self :: new () } }
    };
}

impl_15!()