macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T : Send > FusedIterator for IntoIter < T > { }
    };
}

impl_51!();