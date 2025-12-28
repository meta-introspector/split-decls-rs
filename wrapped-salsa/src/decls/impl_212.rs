macro_rules! deps {
    () => {
        HashEqLike!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl HashEqLike < & str > for String { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , & mut * h) } fn eq (& self , data : & & str) -> bool { self == * data } }
    };
}

impl_212!();