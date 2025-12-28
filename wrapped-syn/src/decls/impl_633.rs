macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_633 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for IntoIter < T > { fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_633!();