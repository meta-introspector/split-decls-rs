macro_rules! deps {
    () => {
        FileSetConfig!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Default for FileSetConfig { fn default () -> Self { FileSetConfig :: builder () . build () } }
    };
}

impl_7!();