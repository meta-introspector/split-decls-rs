macro_rules! deps {
    () => {
        Ascii!();
        Encoding!();
        UniCase!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < S : AsRef < str > > Hash for UniCase < S > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { match self . 0 { Encoding :: Ascii (ref s) => s . hash (hasher) , Encoding :: Unicode (ref s) => s . hash (hasher) , } } }
    };
}

impl_21!()