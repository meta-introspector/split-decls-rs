macro_rules! deps {
    () => {
        Wait!();
    };
}

macro_rules! impl_1248 {
    () => {
        deps!();
        impl Default for Wait { # [inline] fn default () -> Self { Self :: new () } }
    };
}

impl_1248!()