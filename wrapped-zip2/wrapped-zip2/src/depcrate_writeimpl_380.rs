// Generated macro for impl_380 (impl)
macro_rules! Depcrate_writeimpl_380 {
() => {
// Module: crate::write
// Provides: {"impl_380"}
// Dependencies: {}
# [cfg (fuzzing)] impl < 'a > arbitrary :: Arbitrary < 'a > for EncryptWith < 'a > { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { # [cfg (feature = "aes-crypto")] if bool :: arbitrary (u) ? { return Ok (EncryptWith :: Aes { mode : AesMode :: arbitrary (u) ? , password : u . arbitrary :: < & str > () ? , }) ; } Ok (EncryptWith :: ZipCrypto (ZipCryptoKeys :: arbitrary (u) ? , PhantomData ,)) } }
};
}
