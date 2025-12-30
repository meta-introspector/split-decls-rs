// Generated macro for RttEstimate (struct)
macro_rules! Depcrate_load_peak_ewmaRttEstimate {
() => {
// Module: crate::load::peak_ewma
// Provides: {"RttEstimate"}
// Dependencies: {}
# [doc = " Holds the current RTT estimate and the last time this value was updated."] # [derive (Debug)] struct RttEstimate { update_at : Instant , rtt_ns : f64 , }
};
}
