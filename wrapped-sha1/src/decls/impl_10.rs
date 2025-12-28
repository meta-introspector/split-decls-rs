macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Reset for Sha1Core { # [inline] fn reset (& mut self) { * self = Default :: default () ; } }
    };
}

impl_10!();