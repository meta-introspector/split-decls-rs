macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl OutputSizeUser for Sha1Core { type OutputSize = U20 ; }
    };
}

impl_6!();