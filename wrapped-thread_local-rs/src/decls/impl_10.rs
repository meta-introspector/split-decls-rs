macro_rules! deps {
    () => {
        CachedIterMut!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a , T : Send + 'a > ExactSizeIterator for CachedIterMut < 'a , T > { }
    };
}

impl_10!();