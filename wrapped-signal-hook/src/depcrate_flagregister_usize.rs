// Generated macro for register_usize (function)
macro_rules! Depcrate_flagregister_usize {
() => {
// Module: crate::flag
// Provides: {"register_usize"}
// Dependencies: {}
# [doc = " Registers an action to set the flag to the given value whenever the signal arrives."] pub fn register_usize (signal : c_int , flag : Arc < AtomicUsize > , value : usize) -> Result < SigId , Error > { unsafe { low_level :: register (signal , move | | flag . store (value , Ordering :: SeqCst)) } }
};
}
