// Generated macro for impl_441 (impl)
macro_rules! Depcrate_tyimpl_441 {
() => {
// Module: crate::ty
// Provides: {"impl_441"}
// Dependencies: {}
impl Debug for Ty { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Ty") . field ("id" , & self . 0) . field ("kind" , & self . kind ()) . finish () } }
};
}
