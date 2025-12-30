// Generated macro for impl_197 (impl)
macro_rules! Depcrate_tidimpl_197 {
() => {
// Module: crate::tid
// Provides: {"impl_197"}
// Dependencies: {}
impl < C > fmt :: Debug for Tid < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_poisoned () { f . debug_tuple ("Tid") . field (& format_args ! ("<poisoned>")) . finish () } else { f . debug_tuple ("Tid") . field (& format_args ! ("{}" , self . id)) . finish () } } }
};
}
