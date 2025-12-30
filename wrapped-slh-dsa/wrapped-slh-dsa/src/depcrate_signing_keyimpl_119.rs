// Generated macro for impl_119 (impl)
macro_rules! Depcrate_signing_keyimpl_119 {
() => {
// Module: crate::signing_key
// Provides: {"impl_119"}
// Dependencies: {}
impl < N : ArraySize > SkSeed < N > { pub (crate) fn new < R : CryptoRng + ? Sized > (rng : & mut R) -> Self { let mut bytes = Array :: < u8 , N > :: default () ; rng . fill_bytes (bytes . as_mut_slice ()) ; Self (bytes) } }
};
}
