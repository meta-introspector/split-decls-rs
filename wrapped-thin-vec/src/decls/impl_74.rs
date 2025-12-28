macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < 'a , T > ExactSizeIterator for Drain < 'a , T > { }
    };
}

impl_74!()