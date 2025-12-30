// Generated macro for impl_363 (impl)
macro_rules! Depcrate_load_peak_ewmaimpl_363 {
() => {
// Module: crate::load::peak_ewma
// Provides: {"impl_363"}
// Dependencies: {}
impl < S , C > PeakEwma < S , C > { # [doc = " Wraps an `S`-typed service so that its load is tracked by the EWMA of its peak latency."] pub fn new (service : S , default_rtt : Duration , decay_ns : f64 , completion : C) -> Self { debug_assert ! (decay_ns > 0.0 , "decay_ns must be positive") ; Self { service , decay_ns , rtt_estimate : Arc :: new (Mutex :: new (RttEstimate :: new (nanos (default_rtt)))) , completion , } } fn handle (& self) -> Handle { Handle { decay_ns : self . decay_ns , sent_at : Instant :: now () , rtt_estimate : self . rtt_estimate . clone () , } } }
};
}
