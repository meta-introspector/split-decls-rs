// Generated macro for impl_49 (impl)
macro_rules! Depcrate_fieldimpl_49 {
() => {
// Module: crate::field
// Provides: {"impl_49"}
// Dependencies: {}
impl AsField for Field { # [inline] fn as_field (& self , metadata : & Metadata < '_ >) -> Option < Field > { if self . callsite () == metadata . callsite () { Some (self . clone ()) } else { None } } }
};
}
