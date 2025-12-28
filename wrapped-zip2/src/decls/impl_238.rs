macro_rules! deps {
    () => {
        ZipCryptoKeys!();
        EncryptWith!();
        AesMode!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        # [cfg (fuzzing)] impl < 'a > arbitrary :: Arbitrary < 'a > for EncryptWith < 'a > { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { # [cfg (feature = "aes-crypto")] if bool :: arbitrary (u) ? { return Ok (EncryptWith :: Aes { mode : AesMode :: arbitrary (u) ? , password : u . arbitrary :: < & str > () ? , }) ; } Ok (EncryptWith :: ZipCrypto (ZipCryptoKeys :: arbitrary (u) ? , PhantomData ,)) } }
    };
}

impl_238!()