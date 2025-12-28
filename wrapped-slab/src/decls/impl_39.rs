macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for IntoIter < T > { fn len (& self) -> usize { self . len } }
    };
}

impl_39!()