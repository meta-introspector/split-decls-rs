// Generated macro for impl_317 (impl)
macro_rules! Depcrate_limit_rate_serviceimpl_317 {
() => {
// Module: crate::limit::rate::service
// Provides: {"impl_317"}
// Dependencies: {}
impl < T > RateLimit < T > { # [doc = " Create a new rate limiter"] pub fn new (inner : T , rate : Rate) -> Self { let until = Instant :: now () ; let state = State :: Ready { until , rem : rate . num () , } ; RateLimit { inner , rate , state , sleep : Box :: pin (tokio :: time :: sleep_until (until)) , } } # [doc = " Get a reference to the inner service"] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Get a mutable reference to the inner service"] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Consume `self`, returning the inner service"] pub fn into_inner (self) -> T { self . inner } }
};
}
