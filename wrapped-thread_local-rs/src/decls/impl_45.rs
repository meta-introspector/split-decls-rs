macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T : Send > ExactSizeIterator for IterMut < '_ , T > { }
    };
}

impl_45!()