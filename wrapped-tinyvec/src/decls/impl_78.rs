macro_rules! deps {
    () => {
        ArrayVecDrain!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'a , T : 'a + Default > ExactSizeIterator for ArrayVecDrain < 'a , T > { }
    };
}

impl_78!();