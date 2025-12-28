macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl HashMarker for Sha1Core { }
    };
}

impl_3!();