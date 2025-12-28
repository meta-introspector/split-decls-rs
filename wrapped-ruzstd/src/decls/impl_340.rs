macro_rules! deps {
    () => {
        HuffmanTable!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl Default for HuffmanTable { fn default () -> Self { Self :: new () } }
    };
}

impl_340!();