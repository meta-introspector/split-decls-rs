// Generated macro for impl_51 (impl)
macro_rules! Depcrate_hstringimpl_51 {
() => {
// Module: crate::hstring
// Provides: {"impl_51"}
// Dependencies: {}
impl PartialEq < str > for HSTRING { fn eq (& self , other : & str) -> bool { self . iter () . copied () . eq (other . encode_utf16 ()) } }
};
}
