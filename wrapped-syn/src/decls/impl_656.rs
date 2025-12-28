macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        impl < 'a , T > ExactSizeIterator for IterMut < 'a , T > { fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_656!()