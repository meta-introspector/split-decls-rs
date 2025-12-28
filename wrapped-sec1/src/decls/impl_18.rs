macro_rules! deps {
    () => {
        ModulusSize!();
        EncodedPoint!();
        Result!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < Size > fmt :: LowerHex for EncodedPoint < Size > where Size : ModulusSize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:x}" , HexDisplay (self . as_bytes ())) } }
    };
}

impl_18!();