// Generated macro for impl_305 (impl)
macro_rules! Depcrate_backtraceimpl_305 {
() => {
// Module: crate::backtrace
// Provides: {"impl_305"}
// Dependencies: {}
# [stable (feature = "backtrace" , since = "1.65.0")] impl fmt :: Debug for Backtrace { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let capture = match & self . inner { Inner :: Unsupported => return fmt . write_str ("<unsupported>") , Inner :: Disabled => return fmt . write_str ("<disabled>") , Inner :: Captured (c) => & * * c , } ; let frames = & capture . frames [capture . actual_start ..] ; write ! (fmt , "Backtrace ") ? ; let mut dbg = fmt . debug_list () ; for frame in frames { if frame . frame . ip () . is_null () { continue ; } dbg . entries (& frame . symbols) ; } dbg . finish () } }
};
}
