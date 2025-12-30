// Generated macro for impl_370 (impl)
macro_rules! Depcrate_load_peak_ewmaimpl_370 {
() => {
// Module: crate::load::peak_ewma
// Provides: {"impl_370"}
// Dependencies: {}
impl Drop for Handle { fn drop (& mut self) { let recv_at = Instant :: now () ; if let Ok (mut rtt) = self . rtt_estimate . lock () { rtt . update (self . sent_at , recv_at , self . decay_ns) ; } } }
};
}
