macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_529 {
    () => {
        deps!();
        impl < T1 , T2 , CTX > HashStable < CTX > for Result < T1 , T2 > where T1 : HashStable < CTX > , T2 : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { mem :: discriminant (self) . hash_stable (ctx , hasher) ; match * self { Ok (ref x) => x . hash_stable (ctx , hasher) , Err (ref x) => x . hash_stable (ctx , hasher) , } } }
    };
}

impl_529!()