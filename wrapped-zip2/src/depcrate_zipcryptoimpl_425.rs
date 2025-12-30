// Generated macro for impl_425 (impl)
macro_rules! Depcrate_zipcryptoimpl_425 {
() => {
// Module: crate::zipcrypto
// Provides: {"impl_425"}
// Dependencies: {}
impl < W : std :: io :: Write > ZipCryptoWriter < W > { # [allow (unused)] pub (crate) fn finish (mut self , crc32 : u32) -> std :: io :: Result < W > { self . buffer [11] = (crc32 >> 24) as u8 ; for byte in self . buffer . iter_mut () { * byte = self . keys . encrypt_byte (* byte) ; } self . writer . write_all (& self . buffer) ? ; self . writer . flush () ? ; Ok (self . writer) } }
};
}
