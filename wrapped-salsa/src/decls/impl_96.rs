macro_rules! deps {
    () => {
        Durability!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl Default for Durability { fn default () -> Self { Durability :: LOW } }
    };
}

impl_96!();