macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T : Send + Sync > FusedIterator for Iter < '_ , T > { }
    };
}

impl_21!()