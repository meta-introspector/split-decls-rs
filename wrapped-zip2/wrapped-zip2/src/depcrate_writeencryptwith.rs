// Generated macro for EncryptWith (enum)
macro_rules! Depcrate_writeEncryptWith {
() => {
// Module: crate::write
// Provides: {"EncryptWith"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , Eq , PartialEq)] pub (crate) enum EncryptWith < 'k > { # [cfg (feature = "aes-crypto")] Aes { mode : AesMode , password : & 'k str , } , ZipCrypto (ZipCryptoKeys , PhantomData < & 'k () >) , }
};
}
