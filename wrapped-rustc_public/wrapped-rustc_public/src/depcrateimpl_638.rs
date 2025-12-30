// Generated macro for impl_638 (impl)
macro_rules! Depcrateimpl_638 {
() => {
// Module: crate
// Provides: {"impl_638"}
// Dependencies: {}
impl Debug for DefId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DefId") . field ("id" , & self . 0) . field ("name" , & self . name ()) . finish () } }
};
}
