// Generated macro for impl_426 (impl)
macro_rules! Depcrate_zipcryptoimpl_426 {
() => {
// Module: crate::zipcrypto
// Provides: {"impl_426"}
// Dependencies: {}
impl < W : std :: io :: Write > std :: io :: Write for ZipCryptoWriter < W > { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . buffer . extend_from_slice (buf) ; Ok (buf . len ()) } fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
};
}
