// Generated macro for impl_462 (impl)
macro_rules! Depcrate_tyimpl_462 {
() => {
// Module: crate::ty
// Provides: {"impl_462"}
// Dependencies: {}
impl Debug for Span { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Span") . field ("id" , & self . 0) . field ("repr" , & with (| cx | cx . span_to_string (* self))) . finish () } }
};
}
