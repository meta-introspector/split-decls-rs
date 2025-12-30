// Generated macro for impl_11 (impl)
macro_rules! Depcrate_arraysimpl_11 {
() => {
// Module: crate::arrays
// Provides: {"impl_11"}
// Dependencies: {}
impl < const LEN : usize > Deserialize for [u8 ; LEN] { # [cfg (feature = "std")] # [inline] fn tls_deserialize < R : Read > (bytes : & mut R) -> Result < Self , Error > { let mut out = [0u8 ; LEN] ; bytes . read_exact (& mut out) ? ; Ok (out) } }
};
}
