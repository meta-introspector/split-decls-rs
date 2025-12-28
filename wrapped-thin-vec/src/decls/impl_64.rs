macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for IntoIter < T > { }
    };
}

impl_64!();