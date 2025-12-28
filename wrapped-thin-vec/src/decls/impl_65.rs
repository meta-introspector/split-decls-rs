macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < T > core :: iter :: FusedIterator for IntoIter < T > { }
    };
}

impl_65!()