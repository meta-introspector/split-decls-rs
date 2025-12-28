macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for Drain < '_ , T > { fn len (& self) -> usize { self . len } }
    };
}

impl_45!()