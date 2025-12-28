macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T > FusedIterator for Iter < '_ , T > { }
    };
}

impl_38!()