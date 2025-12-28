macro_rules! deps {
    () => {
        Splice!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < I : Iterator > ExactSizeIterator for Splice < '_ , I > { }
    };
}

impl_84!()