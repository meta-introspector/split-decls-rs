macro_rules! deps {
    () => {
        Sha1!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl FixedOutput for Sha1 { # [inline] fn finalize_into (mut self , out : & mut Output < Self >) { self . finalize_inner (out) ; } }
    };
}

impl_89!();