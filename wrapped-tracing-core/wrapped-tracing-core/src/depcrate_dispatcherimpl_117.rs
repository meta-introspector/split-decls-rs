// Generated macro for impl_117 (impl)
macro_rules! Depcrate_dispatcherimpl_117 {
() => {
// Module: crate::dispatcher
// Provides: {"impl_117"}
// Dependencies: {}
impl fmt :: Debug for Dispatch { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . subscriber { Kind :: Scoped (ref s) => f . debug_tuple ("Dispatch::Scoped") . field (& format_args ! ("{:p}" , s)) . finish () , Kind :: Global (s) => f . debug_tuple ("Dispatch::Global") . field (& format_args ! ("{:p}" , s)) . finish () , } } }
};
}
