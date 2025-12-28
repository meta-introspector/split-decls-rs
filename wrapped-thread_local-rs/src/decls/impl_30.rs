macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T : Send > FusedIterator for IntoIter < T > { }
    };
}

impl_30!()