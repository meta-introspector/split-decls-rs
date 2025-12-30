// Generated macro for impl_123 (impl)
macro_rules! Depcrate_signing_keyimpl_123 {
() => {
// Module: crate::signing_key
// Provides: {"impl_123"}
// Dependencies: {}
impl < N : ArraySize > SkPrf < N > { pub (crate) fn new < R : CryptoRng + ? Sized > (rng : & mut R) -> Self { let mut bytes = Array :: < u8 , N > :: default () ; rng . fill_bytes (bytes . as_mut_slice ()) ; Self (bytes) } }
};
}
