// Generated macro for impl_155 (impl)
macro_rules! Depcrate_requestimpl_155 {
() => {
// Module: crate::request
// Provides: {"impl_155"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for CertReq { type Error = der :: Error ; fn try_from (bytes : & 'a [u8]) -> Result < Self , Self :: Error > { Self :: from_der (bytes) } }
};
}
