// Generated macro for CryptoReader (enum)
macro_rules! Depcrate_readCryptoReader {
() => {
// Module: crate::read
// Provides: {"CryptoReader"}
// Dependencies: {}
# [allow (clippy :: large_enum_variant)] pub (crate) enum CryptoReader < 'a , R : Read > { Plaintext (io :: Take < & 'a mut R >) , ZipCrypto (ZipCryptoReaderValid < io :: Take < & 'a mut R > >) , # [cfg (feature = "aes-crypto")] Aes { reader : AesReaderValid < io :: Take < & 'a mut R > > , vendor_version : AesVendorVersion , } , }
};
}
