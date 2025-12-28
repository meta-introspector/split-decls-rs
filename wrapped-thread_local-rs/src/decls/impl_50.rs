macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T : Send > ExactSizeIterator for IntoIter < T > { }
    };
}

impl_50!();