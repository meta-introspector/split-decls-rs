macro_rules! deps {
    () => {
        AsDynError!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'a > AsDynError < 'a > for dyn Error + Send + Sync + 'a { # [inline] fn as_dyn_error (& self) -> & (dyn Error + 'a) { self } }
    };
}

impl_7!()