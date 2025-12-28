macro_rules! deps {
    () => {
        AsDynError!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'a , T : Error + 'a > AsDynError < 'a > for T { # [inline] fn as_dyn_error (& self) -> & (dyn Error + 'a) { self } }
    };
}

impl_4!()