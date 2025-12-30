// Generated macro for impl_171 (impl)
macro_rules! Depcrate_verifying_keyimpl_171 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_171"}
// Dependencies: {}
impl < N : ArraySize > PkSeed < N > { pub (crate) fn new < R : CryptoRng + ? Sized > (rng : & mut R) -> Self { let mut bytes = Array :: < u8 , N > :: default () ; rng . fill_bytes (bytes . as_mut_slice ()) ; Self (bytes) } }
};
}
