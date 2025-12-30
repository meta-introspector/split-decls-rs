// Generated macro for impl_3149 (impl)
macro_rules! Depcrate_sync_mpmc_utilsimpl_3149 {
() => {
// Module: crate::sync::mpmc::utils
// Provides: {"impl_3149"}
// Dependencies: {}
impl Backoff { # [doc = " Creates a new `Backoff`."] pub fn new () -> Self { Backoff { step : Cell :: new (0) } } # [doc = " Backs off using lightweight spinning."] # [doc = ""] # [doc = " This method should be used for retrying an operation because another thread made"] # [doc = " progress. i.e. on CAS failure."] # [inline] pub fn spin_light (& self) { let step = self . step . get () . min (SPIN_LIMIT) ; for _ in 0 .. step . pow (2) { crate :: hint :: spin_loop () ; } self . step . set (self . step . get () + 1) ; } # [doc = " Backs off using heavyweight spinning."] # [doc = ""] # [doc = " This method should be used in blocking loops where parking the thread is not an option."] # [inline] pub fn spin_heavy (& self) { if self . step . get () <= SPIN_LIMIT { for _ in 0 .. self . step . get () . pow (2) { crate :: hint :: spin_loop () } } else { crate :: thread :: yield_now () ; } self . step . set (self . step . get () + 1) ; } }
};
}
