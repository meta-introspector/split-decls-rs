macro_rules! deps {
    () => {
        AesMode!();
        ZipCryptoKeys!();
    };
}

macro_rules! EncryptWith {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub (crate) enum EncryptWith < 'k > { # [cfg (feature = "aes-crypto")] Aes { mode : AesMode , password : & 'k str , } , ZipCrypto (ZipCryptoKeys , PhantomData < & 'k () >) , }
    };
}

EncryptWith!()