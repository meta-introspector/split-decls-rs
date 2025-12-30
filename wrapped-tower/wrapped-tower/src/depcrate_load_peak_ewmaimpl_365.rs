// Generated macro for impl_365 (impl)
macro_rules! Depcrate_load_peak_ewmaimpl_365 {
() => {
// Module: crate::load::peak_ewma
// Provides: {"impl_365"}
// Dependencies: {}
impl < S , C > Load for PeakEwma < S , C > { type Metric = Cost ; fn load (& self) -> Self :: Metric { let pending = Arc :: strong_count (& self . rtt_estimate) as u32 - 1 ; let estimate = self . update_estimate () ; let cost = Cost (estimate * f64 :: from (pending + 1)) ; trace ! ("load estimate={:.0}ms pending={} cost={:?}" , estimate / NANOS_PER_MILLI , pending , cost ,) ; cost } }
};
}
