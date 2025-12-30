// Generated macro for Handle (struct)
macro_rules! Depcrate_load_peak_ewmaHandle {
() => {
// Module: crate::load::peak_ewma
// Provides: {"Handle"}
// Dependencies: {}
# [doc = " Tracks an in-flight request and updates the RTT-estimate on Drop."] # [derive (Debug)] pub struct Handle { sent_at : Instant , decay_ns : f64 , rtt_estimate : Arc < Mutex < RttEstimate > > , }
};
}
