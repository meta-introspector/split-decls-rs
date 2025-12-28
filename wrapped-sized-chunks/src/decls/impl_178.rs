macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < 'a , A : Eq + 'a , const N : usize > Eq for Slice < 'a , A , N > { }
    };
}

impl_178!()