macro_rules! deps {
    () => {
        AsDynError!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < 'a > AsDynError < 'a > for dyn Error + Send + 'a { # [inline] fn as_dyn_error (& self) -> & (dyn Error + 'a) { self } }
    };
}

impl_6!()