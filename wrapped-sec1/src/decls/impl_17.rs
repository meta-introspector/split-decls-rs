macro_rules! deps {
    () => {
        ModulusSize!();
        EncodedPoint!();
        Result!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < Size > fmt :: Display for EncodedPoint < Size > where Size : ModulusSize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{self:X}") } }
    };
}

impl_17!()