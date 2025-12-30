// Generated macro for impl_3124 (impl)
macro_rules! Depcrate_sync_mpmc_listimpl_3124 {
() => {
// Module: crate::sync::mpmc::list
// Provides: {"impl_3124"}
// Dependencies: {}
impl < T > Slot < T > { # [doc = " Waits until a message is written into the slot."] fn wait_write (& self) { let backoff = Backoff :: new () ; while self . state . load (Ordering :: Acquire) & WRITE == 0 { backoff . spin_heavy () ; } } }
};
}
