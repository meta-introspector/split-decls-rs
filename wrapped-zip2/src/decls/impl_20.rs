macro_rules! deps {
    () => {
        Cipher!();
        Aes128!();
        AesKind!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl AesKind for Aes128 { type Key = [u8 ; 16] ; type Cipher = aes :: Aes128 ; }
    };
}

impl_20!();