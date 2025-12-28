macro_rules! deps {
    () => {
        MockTask!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Default for MockTask { fn default () -> Self { Self :: new () } }
    };
}

impl_50!()