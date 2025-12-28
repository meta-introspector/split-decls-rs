macro_rules! deps {
    () => {
        FixupContext!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl Clone for FixupContext { fn clone (& self) -> Self { * self } }
    };
}

impl_282!();