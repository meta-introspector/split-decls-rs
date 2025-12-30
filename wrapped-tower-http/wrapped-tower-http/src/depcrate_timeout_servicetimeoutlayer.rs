// Generated macro for TimeoutLayer (struct)
macro_rules! Depcrate_timeout_serviceTimeoutLayer {
() => {
// Module: crate::timeout::service
// Provides: {"TimeoutLayer"}
// Dependencies: {}
# [doc = " Layer that applies the [`Timeout`] middleware which apply a timeout to requests."] # [doc = ""] # [doc = " See the [module docs](super) for an example."] # [derive (Debug , Clone , Copy)] pub struct TimeoutLayer { timeout : Duration , status_code : StatusCode , }
};
}
