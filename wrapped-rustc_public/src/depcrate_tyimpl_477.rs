// Generated macro for impl_477 (impl)
macro_rules! Depcrate_tyimpl_477 {
() => {
// Module: crate::ty
// Provides: {"impl_477"}
// Dependencies: {}
impl Debug for Span { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Span") . field ("id" , & self . 0) . field ("repr" , & with (| cx | cx . span_to_string (* self))) . finish () } }
};
}
