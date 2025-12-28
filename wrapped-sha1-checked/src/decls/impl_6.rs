macro_rules! deps {
    () => {
        Sha1!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl HashMarker for Sha1 { }
    };
}

impl_6!()