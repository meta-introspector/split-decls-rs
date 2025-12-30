// Generated macro for impl_120 (impl)
macro_rules! Depcrate_dispatcherimpl_120 {
() => {
// Module: crate::dispatcher
// Provides: {"impl_120"}
// Dependencies: {}
impl fmt :: Debug for WeakDispatch { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . subscriber { Kind :: Scoped (ref s) => f . debug_tuple ("WeakDispatch::Scoped") . field (& format_args ! ("{:p}" , s)) . finish () , Kind :: Global (s) => f . debug_tuple ("WeakDispatch::Global") . field (& format_args ! ("{:p}" , s)) . finish () , } } }
};
}
