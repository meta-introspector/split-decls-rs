// Generated macro for impl_211 (impl)
macro_rules! Depcrate_paximpl_211 {
() => {
// Module: crate::pax
// Provides: {"impl_211"}
// Dependencies: {}
impl < 'entry > PaxExtensions < 'entry > { # [doc = " Create new pax extensions iterator from the given entry data."] pub fn new (a : & 'entry [u8]) -> Self { fn is_newline (a : & u8) -> bool { * a == b'\n' } PaxExtensions { data : a . split (is_newline) , } } }
};
}
