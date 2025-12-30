// Generated macro for impl_158 (impl)
macro_rules! Depcrate_intrinsicimpl_158 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_158"}
// Dependencies: {}
impl fmt :: Display for GovernedBy { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Predicated => write ! (f , " (governed by `pg`)") , Self :: PredicatedNonFaulting => write ! (f , " (governed by `pg`, the first-fault register (`FFR`) \
                and non-faulting behaviour)") , Self :: PredicatedFirstFaulting => write ! (f , " (governed by `pg`, the first-fault register (`FFR`) \
                and first-faulting behaviour)") , } } }
};
}
