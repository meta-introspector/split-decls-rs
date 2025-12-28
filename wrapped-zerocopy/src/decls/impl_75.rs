macro_rules! deps {
    () => {
        AsAddress!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < T : ? Sized > AsAddress for NonNull < T > { # [inline (always)] fn addr (self) -> usize { AsAddress :: addr (self . as_ptr ()) } }
    };
}

impl_75!()