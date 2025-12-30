// Generated macro for impl_64 (impl)
macro_rules! Depcrate_child_wrapperimpl_64 {
() => {
// Module: crate::child_wrapper
// Provides: {"impl_64"}
// Dependencies: {}
impl fmt :: Display for ExitStatusWrapper { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { ExitStatusEnum :: Std (ref es) => fmt :: Display :: fmt (es , f) , } } }
};
}
