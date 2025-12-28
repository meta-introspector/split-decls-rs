macro_rules! deps {
    () => {
        WaitPtr!();
    };
}

macro_rules! impl_1243 {
    () => {
        deps!();
        impl Default for WaitPtr { # [inline] fn default () -> Self { Self :: new (ptr :: null_mut ()) } }
    };
}

impl_1243!()