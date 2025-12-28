macro_rules! deps {
    () => {
        CryptoReader!();
        AesVendorVersion!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < 'a , R : Read > CryptoReader < 'a , R > { # [doc = " Consumes this decoder, returning the underlying reader."] pub fn into_inner (self) -> io :: Take < & 'a mut R > { match self { CryptoReader :: Plaintext (r) => r , CryptoReader :: ZipCrypto (r) => r . into_inner () , # [cfg (feature = "aes-crypto")] CryptoReader :: Aes { reader : r , .. } => r . into_inner () , } } # [doc = " Returns `true` if the data is encrypted using AE2."] pub const fn is_ae2_encrypted (& self) -> bool { # [cfg (feature = "aes-crypto")] return matches ! (self , CryptoReader :: Aes { vendor_version : AesVendorVersion :: Ae2 , .. }) ; # [cfg (not (feature = "aes-crypto"))] false } }
    };
}

impl_82!();