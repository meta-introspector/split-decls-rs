macro_rules! deps {
    () => {
        Tag!();
        TaggedRef!();
    };
}

macro_rules! impl_588 {
    () => {
        deps!();
        impl < P , T : Tag > Hash for TaggedRef < '_ , P , T > { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . packed . hash (state) ; } }
    };
}

impl_588!()