// Generated macro for MaybeEncrypted (enum)
macro_rules! Depcrate_writeMaybeEncrypted {
() => {
// Module: crate::write
// Provides: {"MaybeEncrypted"}
// Dependencies: {}
enum MaybeEncrypted < W > { Unencrypted (W) , # [cfg (feature = "aes-crypto")] Aes (AesWriter < W >) , ZipCrypto (crate :: zipcrypto :: ZipCryptoWriter < W >) , }
};
}
