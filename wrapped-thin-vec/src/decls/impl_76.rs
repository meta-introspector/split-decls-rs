macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T > core :: iter :: FusedIterator for Drain < '_ , T > { }
    };
}

impl_76!();