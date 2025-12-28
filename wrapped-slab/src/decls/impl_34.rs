macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T > FusedIterator for IntoIter < T > { }
    };
}

impl_34!()