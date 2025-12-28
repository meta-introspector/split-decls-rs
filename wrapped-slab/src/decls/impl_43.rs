macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for Iter < '_ , T > { fn len (& self) -> usize { self . len } }
    };
}

impl_43!()