macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > FusedIterator for IterMut < 'a , A , N > where A : 'a { }
    };
}

impl_155!()