// Generated macro for RateLimit (struct)
macro_rules! Depcrate_limit_rate_serviceRateLimit {
() => {
// Module: crate::limit::rate::service
// Provides: {"RateLimit"}
// Dependencies: {}
# [doc = " Enforces a rate limit on the number of requests the underlying"] # [doc = " service can handle over a period of time."] # [derive (Debug)] pub struct RateLimit < T > { inner : T , rate : Rate , state : State , sleep : Pin < Box < Sleep > > , }
};
}
