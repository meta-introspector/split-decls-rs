macro_rules! deps {
    () => {
        Sha1!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl OutputSizeUser for Sha1 { type OutputSize = U20 ; }
    };
}

impl_14!()