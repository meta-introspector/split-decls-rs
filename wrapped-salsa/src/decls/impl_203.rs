macro_rules! deps {
    () => {
        HashEqLike!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < T > HashEqLike < T > for & T where T : Hash + Eq , { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (* self , & mut * h) ; } fn eq (& self , data : & T) -> bool { * * self == * data } }
    };
}

impl_203!();