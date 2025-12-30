// Generated macro for impl_648 (impl)
macro_rules! Depcrate_timeoutimpl_648 {
() => {
// Module: crate::timeout
// Provides: {"impl_648"}
// Dependencies: {}
impl < T > Timeout < T > { # [doc = " Creates a new [`Timeout`]"] pub const fn new (inner : T , timeout : Duration) -> Self { Timeout { inner , timeout } } # [doc = " Get a reference to the inner service"] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Get a mutable reference to the inner service"] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Consume `self`, returning the inner service"] pub fn into_inner (self) -> T { self . inner } }
};
}
