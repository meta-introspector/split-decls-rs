macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T > FusedIterator for Drain < '_ , T > { }
    };
}

impl_46!()