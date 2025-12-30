// Generated macro for impl_50 (impl)
macro_rules! Depcrate_fieldimpl_50 {
() => {
// Module: crate::field
// Provides: {"impl_50"}
// Dependencies: {}
impl AsField for & Field { # [inline] fn as_field (& self , metadata : & Metadata < '_ >) -> Option < Field > { if self . callsite () == metadata . callsite () { Some ((* self) . clone ()) } else { None } } }
};
}
