macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_643 {
    () => {
        deps!();
        impl < 'a , T > ExactSizeIterator for Iter < 'a , T > { fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_643!()