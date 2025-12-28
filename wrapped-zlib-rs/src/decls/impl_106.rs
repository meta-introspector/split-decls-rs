macro_rules! deps {
    () => {
        Crc32Fold!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl Default for Crc32Fold { fn default () -> Self { Self :: new () } }
    };
}

impl_106!();