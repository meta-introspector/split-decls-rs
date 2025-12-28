macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < T > FusedIterator for Drain < '_ , T > { }
    };
}

impl_52!()