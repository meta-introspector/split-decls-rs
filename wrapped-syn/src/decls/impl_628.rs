macro_rules! deps {
    () => {
        IntoPairs!();
    };
}

macro_rules! impl_628 {
    () => {
        deps!();
        impl < T , P > ExactSizeIterator for IntoPairs < T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
    };
}

impl_628!()