// Generated macro for impl_366 (impl)
macro_rules! Depcrate_load_peak_ewmaimpl_366 {
() => {
// Module: crate::load::peak_ewma
// Provides: {"impl_366"}
// Dependencies: {}
impl < S , C > PeakEwma < S , C > { fn update_estimate (& self) -> f64 { let mut rtt = self . rtt_estimate . lock () . expect ("peak ewma prior_estimate") ; rtt . decay (self . decay_ns) } }
};
}
