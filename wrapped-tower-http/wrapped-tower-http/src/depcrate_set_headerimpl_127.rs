// Generated macro for impl_127 (impl)
macro_rules! Depcrate_set_headerimpl_127 {
() => {
// Module: crate::set_header
// Provides: {"impl_127"}
// Dependencies: {}
impl < F , T > MakeHeaderValue < T > for F where F : FnMut (& T) -> Option < HeaderValue > , { fn make_header_value (& mut self , message : & T) -> Option < HeaderValue > { self (message) } }
};
}
