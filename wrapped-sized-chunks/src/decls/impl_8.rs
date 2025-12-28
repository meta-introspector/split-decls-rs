macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a , A , T > ExactSizeIterator for Drain < 'a , A , T > { }
    };
}

impl_8!()