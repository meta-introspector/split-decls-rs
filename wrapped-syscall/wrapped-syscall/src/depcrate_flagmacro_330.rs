// Generated macro for macro_330 (macro)
macro_rules! Depcrate_flagmacro_330 {
() => {
// Module: crate::flag
// Provides: {"macro_330"}
// Dependencies: {}
bitflags ! { pub struct SigcontrolFlags : usize { # [doc = " Prevents the kernel from jumping the context to the signal trampoline, but otherwise"] # [doc = " has absolutely no effect on which signals are blocked etc. Meant to be used for"] # [doc = " short-lived critical sections inside libc."] const INHIBIT_DELIVERY = 1 ; } }
};
}
