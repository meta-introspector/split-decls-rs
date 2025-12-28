macro_rules! deps {
    () => {
        Aes128!();
        AesKind!();
        Cipher!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl AesKind for Aes128 { type Key = [u8 ; 16] ; type Cipher = aes :: Aes128 ; }
    };
}

impl_20!()