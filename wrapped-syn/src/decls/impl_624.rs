macro_rules! deps {
    () => {
        PairsMut!();
    };
}

macro_rules! impl_624 {
    () => {
        deps!();
        impl < 'a , T , P > ExactSizeIterator for PairsMut < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
    };
}

impl_624!();