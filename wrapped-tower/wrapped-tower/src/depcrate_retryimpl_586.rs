// Generated macro for impl_586 (impl)
macro_rules! Depcrate_retryimpl_586 {
() => {
// Module: crate::retry
// Provides: {"impl_586"}
// Dependencies: {}
impl < P , S > Retry < P , S > { # [doc = " Retry the inner service depending on this [`Policy`]."] pub const fn new (policy : P , service : S) -> Self { Retry { policy , service } } # [doc = " Get a reference to the inner service"] pub fn get_ref (& self) -> & S { & self . service } # [doc = " Get a mutable reference to the inner service"] pub fn get_mut (& mut self) -> & mut S { & mut self . service } # [doc = " Consume `self`, returning the inner service"] pub fn into_inner (self) -> S { self . service } }
};
}
