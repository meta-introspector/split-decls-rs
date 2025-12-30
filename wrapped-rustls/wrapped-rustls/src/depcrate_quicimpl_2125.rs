// Generated macro for impl_2125 (impl)
macro_rules! Depcrate_quicimpl_2125 {
() => {
// Module: crate::quic
// Provides: {"impl_2125"}
// Dependencies: {}
impl From < & [u8] > for Tag { fn from (value : & [u8]) -> Self { let mut array = [0u8 ; TAG_LEN] ; array . copy_from_slice (value) ; Self (array) } }
};
}
