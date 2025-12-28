macro_rules! deps {
    () => {
        EncodedPoint!();
        ModulusSize!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < Size > Hash for EncodedPoint < Size > where Size : ModulusSize , { fn hash < H : Hasher > (& self , state : & mut H) { self . as_bytes () . hash (state) } }
    };
}

impl_12!();