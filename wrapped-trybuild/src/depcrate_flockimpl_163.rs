// Generated macro for impl_163 (impl)
macro_rules! Depcrate_flockimpl_163 {
() => {
// Module: crate::flock
// Provides: {"impl_163"}
// Dependencies: {}
impl Guard { fn acquire () -> Self { Guard :: Locked (LOCK . lock () . unwrap_or_else (PoisonError :: into_inner)) } }
};
}
