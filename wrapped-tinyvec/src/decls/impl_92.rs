macro_rules! deps {
    () => {
        SliceVecDrain!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < 'p , 's , T : Default > FusedIterator for SliceVecDrain < 'p , 's , T > { }
    };
}

impl_92!();