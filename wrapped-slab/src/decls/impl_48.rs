macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T > FusedIterator for IterMut < '_ , T > { }
    };
}

impl_48!();