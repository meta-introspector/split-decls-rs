macro_rules! deps {
    () => {
        TomlIntegerFormat!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Default for TomlIntegerFormat { fn default () -> Self { Self :: new () } }
    };
}

impl_3!();