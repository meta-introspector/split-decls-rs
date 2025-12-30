// Generated macro for register (function)
macro_rules! Depcrate_flagregister {
() => {
// Module: crate::flag
// Provides: {"register"}
// Dependencies: {}
# [doc = " Registers an action to set the flag to `true` whenever the given signal arrives."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the signal is one of the forbidden."] pub fn register (signal : c_int , flag : Arc < AtomicBool >) -> Result < SigId , Error > { unsafe { low_level :: register (signal , move | | flag . store (true , Ordering :: SeqCst)) } }
};
}
