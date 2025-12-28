macro_rules! deps {
    () => {
        HashEqLike!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl < A , T : Hash + Eq + PartialEq < A > > HashEqLike < & [A] > for Vec < T > { fn hash < H : Hasher > (& self , h : & mut H) { Hash :: hash (self , h) ; } fn eq (& self , data : & & [A]) -> bool { self . len () == data . len () && data . iter () . enumerate () . all (| (i , a) | & self [i] == a) } }
    };
}

impl_214!()