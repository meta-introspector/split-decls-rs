// Generated macro for impl_456 (impl)
macro_rules! Depcrate_tyimpl_456 {
() => {
// Module: crate::ty
// Provides: {"impl_456"}
// Dependencies: {}
impl Debug for Ty { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Ty") . field ("id" , & self . 0) . field ("kind" , & self . kind ()) . finish () } }
};
}
