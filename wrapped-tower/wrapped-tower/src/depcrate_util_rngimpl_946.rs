// Generated macro for impl_946 (impl)
macro_rules! Depcrate_util_rngimpl_946 {
() => {
// Module: crate::util::rng
// Provides: {"impl_946"}
// Dependencies: {}
impl < R : Rng + ? Sized > Rng for Box < R > { fn next_u64 (& mut self) -> u64 { (* * self) . next_u64 () } }
};
}
