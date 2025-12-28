macro_rules! impl_244 {
    () => {
        impl Hash for Member { fn hash < H : Hasher > (& self , state : & mut H) { match self { Member :: Named (m) => m . hash (state) , Member :: Unnamed (m) => m . hash (state) , } } }
    };
}

impl_244!();