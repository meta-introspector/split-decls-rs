// Generated macro for impl_2758 (impl)
macro_rules! Depcrate_panicimpl_2758 {
() => {
// Module: crate::panic
// Provides: {"impl_2758"}
// Dependencies: {}
# [stable (feature = "panic_hook_display" , since = "1.26.0")] impl fmt :: Display for PanicHookInfo < '_ > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("panicked at ") ? ; self . location . fmt (formatter) ? ; if let Some (payload) = self . payload_as_str () { formatter . write_str (":\n") ? ; formatter . write_str (payload) ? ; } Ok (()) } }
};
}
