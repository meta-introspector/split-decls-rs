// Generated macro for impl_21 (impl)
macro_rules! Depcrate_bstrimpl_21 {
() => {
// Module: crate::bstr
// Provides: {"impl_21"}
// Dependencies: {}
impl < T : AsRef < str > + ? Sized > PartialEq < T > for BSTR { fn eq (& self , other : & T) -> bool { self . iter () . copied () . eq (other . as_ref () . encode_utf16 ()) } }
};
}
