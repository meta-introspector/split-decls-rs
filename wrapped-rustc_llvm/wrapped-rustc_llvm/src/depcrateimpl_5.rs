// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl RustString { pub fn build_byte_buffer (closure : impl FnOnce (& Self)) -> Vec < u8 > { let buf = RustStringInner :: default () ; closure (buf . as_opaque ()) ; buf . into_inner () } }
};
}
