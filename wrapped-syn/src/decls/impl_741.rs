macro_rules! deps {
    () => {
        TokenStreamHelper!();
        TokenTreeHelper!();
    };
}

macro_rules! impl_741 {
    () => {
        deps!();
        impl < 'a > Hash for TokenStreamHelper < 'a > { fn hash < H : Hasher > (& self , state : & mut H) { let tokens = self . 0 . clone () . into_iter () ; tokens . clone () . count () . hash (state) ; for tt in tokens { TokenTreeHelper (& tt) . hash (state) ; } } }
    };
}

impl_741!()