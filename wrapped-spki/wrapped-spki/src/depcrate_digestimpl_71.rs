// Generated macro for impl_71 (impl)
macro_rules! Depcrate_digestimpl_71 {
() => {
// Module: crate::digest
// Provides: {"impl_71"}
// Dependencies: {}
impl < D > Writer for DigestWriter < '_ , D > where D : Digest , { fn write (& mut self , slice : & [u8]) -> Result < () > { self . 0 . update (slice) ; Ok (()) } }
};
}
