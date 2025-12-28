macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T : Send > FusedIterator for IterMut < '_ , T > { }
    };
}

impl_25!()