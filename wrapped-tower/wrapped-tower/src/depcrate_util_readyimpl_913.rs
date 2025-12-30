// Generated macro for impl_913 (impl)
macro_rules! Depcrate_util_readyimpl_913 {
() => {
// Module: crate::util::ready
// Provides: {"impl_913"}
// Dependencies: {}
impl < T , Request > ReadyOneshot < T , Request > where T : Service < Request > , { # [allow (missing_docs)] pub const fn new (service : T) -> Self { Self { inner : Some (service) , _p : PhantomData , } } }
};
}
