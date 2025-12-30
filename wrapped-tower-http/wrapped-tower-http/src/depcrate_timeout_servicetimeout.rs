// Generated macro for Timeout (struct)
macro_rules! Depcrate_timeout_serviceTimeout {
() => {
// Module: crate::timeout::service
// Provides: {"Timeout"}
// Dependencies: {}
# [doc = " Middleware which apply a timeout to requests."] # [doc = ""] # [doc = " See the [module docs](super) for an example."] # [derive (Debug , Clone , Copy)] pub struct Timeout < S > { inner : S , timeout : Duration , status_code : StatusCode , }
};
}
