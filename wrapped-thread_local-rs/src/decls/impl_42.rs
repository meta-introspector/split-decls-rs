macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T : Send + Sync > FusedIterator for Iter < '_ , T > { }
    };
}

impl_42!()