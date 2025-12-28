macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Hash for Uuid { fn hash < H : Hasher > (& self , state : & mut H) { state . write (& self . 0) ; } }
    };
}

impl_27!()