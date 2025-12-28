macro_rules! deps {
    () => {
        UniCase!();
        Ascii!();
        Encoding!();
        Unicode!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < S : AsRef < str > > Hash for UniCase < S > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { match self . 0 { Encoding :: Ascii (ref s) => s . hash (hasher) , Encoding :: Unicode (ref s) => s . hash (hasher) , } } }
    };
}

impl_48!()