// Generated macro for impl_309 (impl)
macro_rules! Depcrate_limit_rate_rateimpl_309 {
() => {
// Module: crate::limit::rate::rate
// Provides: {"impl_309"}
// Dependencies: {}
impl Rate { # [doc = " Create a new rate."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function panics if `num` or `per` is 0."] pub const fn new (num : u64 , per : Duration) -> Self { assert ! (num > 0) ; assert ! (per . as_nanos () > 0) ; Rate { num , per } } pub (crate) fn num (& self) -> u64 { self . num } pub (crate) fn per (& self) -> Duration { self . per } }
};
}
