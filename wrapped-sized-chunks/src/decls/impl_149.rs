macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > FusedIterator for Iter < 'a , A , N > { }
    };
}

impl_149!()