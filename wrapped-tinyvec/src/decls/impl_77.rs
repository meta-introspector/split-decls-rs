macro_rules! deps {
    () => {
        ArrayVecDrain!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < 'a , T : 'a + Default > FusedIterator for ArrayVecDrain < 'a , T > { }
    };
}

impl_77!()