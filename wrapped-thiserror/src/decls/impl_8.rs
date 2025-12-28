macro_rules! deps {
    () => {
        AsDynError!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a > AsDynError < 'a > for dyn Error + Send + Sync + UnwindSafe + 'a { # [inline] fn as_dyn_error (& self) -> & (dyn Error + 'a) { self } }
    };
}

impl_8!()