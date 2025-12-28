macro_rules! deps {
    () => {
        Sha1!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl FixedOutputReset for Sha1 { # [inline] fn finalize_into_reset (& mut self , out : & mut Output < Self >) { self . finalize_inner (out) ; Reset :: reset (self) ; } }
    };
}

impl_90!();