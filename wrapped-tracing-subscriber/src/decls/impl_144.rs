macro_rules! deps {
    () => {
        Identity!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl Identity { # [doc = " Returns a new `Identity` layer."] pub fn new () -> Self { Self { _p : () } } }
    };
}

impl_144!()