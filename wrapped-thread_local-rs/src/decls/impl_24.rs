macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T : Send > ExactSizeIterator for IterMut < '_ , T > { }
    };
}

impl_24!()