macro_rules! deps {
    () => {
        AesCtrZipKeyStream!();
        AesKind!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < C > fmt :: Debug for AesCtrZipKeyStream < C > where C : AesKind , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "AesCtrZipKeyStream<{}>(counter: {})" , any :: type_name ::< C > () , self . counter) } }
    };
}

impl_24!()