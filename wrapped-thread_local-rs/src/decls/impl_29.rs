macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T : Send > ExactSizeIterator for IntoIter < T > { }
    };
}

impl_29!()