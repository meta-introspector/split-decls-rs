macro_rules! deps {
    () => {
        Aes192!();
        AesKind!();
        Cipher!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl AesKind for Aes192 { type Key = [u8 ; 24] ; type Cipher = aes :: Aes192 ; }
    };
}

impl_21!()