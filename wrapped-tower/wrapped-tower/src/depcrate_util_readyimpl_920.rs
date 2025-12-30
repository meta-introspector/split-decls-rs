// Generated macro for impl_920 (impl)
macro_rules! Depcrate_util_readyimpl_920 {
() => {
// Module: crate::util::ready
// Provides: {"impl_920"}
// Dependencies: {}
impl < T , Request > fmt :: Debug for Ready < '_ , T , Request > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("Ready") . field (& self . 0) . finish () } }
};
}
