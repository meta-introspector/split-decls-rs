macro_rules! deps {
    () => {
        AsAddress!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < T : ? Sized > AsAddress for & T { # [inline (always)] fn addr (self) -> usize { let ptr : * const T = self ; AsAddress :: addr (ptr) } }
    };
}

impl_73!();