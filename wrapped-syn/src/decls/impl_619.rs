macro_rules! deps {
    () => {
        Pairs!();
    };
}

macro_rules! impl_619 {
    () => {
        deps!();
        impl < 'a , T , P > ExactSizeIterator for Pairs < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
    };
}

impl_619!();