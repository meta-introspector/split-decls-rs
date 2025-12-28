macro_rules! deps {
    () => {
        Aes256!();
        AesKind!();
        Cipher!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl AesKind for Aes256 { type Key = [u8 ; 32] ; type Cipher = aes :: Aes256 ; }
    };
}

impl_22!();