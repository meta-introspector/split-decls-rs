macro_rules! deps {
    () => {
        Result!();
        ModulusSize!();
        EncodedPoint!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < Size > Debug for EncodedPoint < Size > where Size : ModulusSize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "EncodedPoint({:?})" , self . coordinates ()) } }
    };
}

impl_9!()