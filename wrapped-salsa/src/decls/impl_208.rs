macro_rules! deps {
    () => {
        HashEqLike!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < 'a , T > HashEqLike < & 'a T > for Arc < T > where T : ? Sized + Hash + Eq , Arc < T > : From < & 'a T > , { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (& * * self , & mut * h) } fn eq (& self , data : & & T) -> bool { * * self == * * data } }
    };
}

impl_208!()