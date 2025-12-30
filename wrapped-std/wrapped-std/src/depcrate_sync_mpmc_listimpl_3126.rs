// Generated macro for impl_3126 (impl)
macro_rules! Depcrate_sync_mpmc_listimpl_3126 {
() => {
// Module: crate::sync::mpmc::list
// Provides: {"impl_3126"}
// Dependencies: {}
impl < T > Block < T > { # [doc = " Creates an empty block."] fn new () -> Box < Block < T > > { unsafe { Box :: new_zeroed () . assume_init () } } # [doc = " Waits until the next pointer is set."] fn wait_next (& self) -> * mut Block < T > { let backoff = Backoff :: new () ; loop { let next = self . next . load (Ordering :: Acquire) ; if ! next . is_null () { return next ; } backoff . spin_heavy () ; } } # [doc = " Sets the `DESTROY` bit in slots starting from `start` and destroys the block."] unsafe fn destroy (this : * mut Block < T > , start : usize) { for i in start .. BLOCK_CAP - 1 { let slot = unsafe { (* this) . slots . get_unchecked (i) } ; if slot . state . load (Ordering :: Acquire) & READ == 0 && slot . state . fetch_or (DESTROY , Ordering :: AcqRel) & READ == 0 { return ; } } drop (unsafe { Box :: from_raw (this) }) ; } }
};
}
