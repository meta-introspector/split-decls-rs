// Generated macro for impl_51 (impl)
macro_rules! Depcrate_active_queryimpl_51 {
() => {
// Module: crate::active_query
// Provides: {"impl_51"}
// Dependencies: {}
impl fmt :: Debug for Backtrace { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "Backtrace ") ? ; let mut dbg = fmt . debug_list () ; for frame in & self . 0 { dbg . entry (& frame) ; } dbg . finish () } }
};
}
