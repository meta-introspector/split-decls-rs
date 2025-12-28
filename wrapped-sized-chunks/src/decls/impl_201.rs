macro_rules! deps {
    () => {
        SliceMut!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < 'a , A : Hash + 'a , const N : usize > Hash for SliceMut < 'a , A , N > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { for item in self { item . hash (hasher) } } }
    };
}

impl_201!()