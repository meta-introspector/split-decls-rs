macro_rules! deps {
    () => {
        HashEqLike!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl < const N : usize , A , T : Hash + Eq + PartialEq < A > > HashEqLike < [A ; N] > for Vec < T > { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , h) ; } fn eq (& self , data : & [A ; N]) -> bool { self . len () == data . len () && data . iter () . enumerate () . all (| (i , a) | & self [i] == a) } }
    };
}

impl_216!();