// Generated macro for impl_428 (impl)
macro_rules! Depcrate_zipcryptoimpl_428 {
() => {
// Module: crate::zipcrypto
// Provides: {"impl_428"}
// Dependencies: {}
impl < R : std :: io :: Read > std :: io :: Read for ZipCryptoReaderValid < R > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let n = self . reader . file . read (buf) ? ; for byte in buf . iter_mut () . take (n) { * byte = self . reader . keys . decrypt_byte (* byte) ; } Ok (n) } }
};
}
