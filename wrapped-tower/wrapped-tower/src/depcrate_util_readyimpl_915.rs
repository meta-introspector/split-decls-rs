// Generated macro for impl_915 (impl)
macro_rules! Depcrate_util_readyimpl_915 {
() => {
// Module: crate::util::ready
// Provides: {"impl_915"}
// Dependencies: {}
impl < T , Request > fmt :: Debug for ReadyOneshot < T , Request > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("ReadyOneshot") . field ("inner" , & self . inner) . finish () } }
};
}
