macro_rules! deps {
    () => {
        SliceMut!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < 'a , A : Eq + 'a , const N : usize > Eq for SliceMut < 'a , A , N > { }
    };
}

impl_197!();