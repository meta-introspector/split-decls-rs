// Generated macro for impl_13 (impl)
macro_rules! Depcrate_arraysimpl_13 {
() => {
// Module: crate::arrays
// Provides: {"impl_13"}
// Dependencies: {}
impl < const LEN : usize > SerializeBytes for [u8 ; LEN] { fn tls_serialize (& self) -> Result < Vec < u8 > , Error > { Ok (self . to_vec ()) } }
};
}
