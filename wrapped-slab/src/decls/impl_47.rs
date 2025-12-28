macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for IterMut < '_ , T > { fn len (& self) -> usize { self . len } }
    };
}

impl_47!();