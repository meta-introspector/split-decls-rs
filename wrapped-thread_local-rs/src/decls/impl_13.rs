macro_rules! deps {
    () => {
        CachedIntoIter!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T : Send > ExactSizeIterator for CachedIntoIter < T > { }
    };
}

impl_13!()