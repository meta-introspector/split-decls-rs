macro_rules! deps {
    () => {
        AsAddress!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < T : ? Sized > AsAddress for * mut T { # [inline (always)] fn addr (self) -> usize { let ptr : * const T = self ; AsAddress :: addr (ptr) } }
    };
}

impl_77!()