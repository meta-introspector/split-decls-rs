macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Default for Args { fn default () -> Self { Self :: new () } }
    };
}

impl_17!()