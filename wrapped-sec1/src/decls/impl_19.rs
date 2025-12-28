macro_rules! deps {
    () => {
        ModulusSize!();
        Result!();
        EncodedPoint!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < Size > fmt :: UpperHex for EncodedPoint < Size > where Size : ModulusSize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:X}" , HexDisplay (self . as_bytes ())) } }
    };
}

impl_19!()