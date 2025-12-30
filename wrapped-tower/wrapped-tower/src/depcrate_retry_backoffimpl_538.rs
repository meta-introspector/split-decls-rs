// Generated macro for impl_538 (impl)
macro_rules! Depcrate_retry_backoffimpl_538 {
() => {
// Module: crate::retry::backoff
// Provides: {"impl_538"}
// Dependencies: {}
impl < R > Backoff for ExponentialBackoff < R > where R : Rng , { type Future = tokio :: time :: Sleep ; fn next_backoff (& mut self) -> Self :: Future { let base = self . base () ; let next = base + self . jitter (base) ; self . iterations += 1 ; tokio :: time :: sleep (next) } }
};
}
