macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > ExactSizeIterator for IterMut < 'a , A , N > where A : 'a { }
    };
}

impl_154!()