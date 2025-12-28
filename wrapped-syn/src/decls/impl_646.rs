macro_rules! deps {
    () => {
        PrivateIter!();
    };
}

macro_rules! impl_646 {
    () => {
        deps!();
        impl < 'a , T , P > ExactSizeIterator for PrivateIter < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
    };
}

impl_646!();