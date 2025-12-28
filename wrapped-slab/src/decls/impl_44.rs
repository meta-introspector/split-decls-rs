macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T > FusedIterator for Iter < '_ , T > { }
    };
}

impl_44!()