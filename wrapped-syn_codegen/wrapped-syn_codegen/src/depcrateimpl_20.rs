// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl fmt :: Debug for AstItem { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("AstItem") . field ("item" , & self . item) . field ("features" , & self . features . to_string ()) . finish () } }
};
}
