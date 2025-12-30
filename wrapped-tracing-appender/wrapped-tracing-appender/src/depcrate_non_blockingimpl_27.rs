// Generated macro for impl_27 (impl)
macro_rules! Depcrate_non_blockingimpl_27 {
() => {
// Module: crate::non_blocking
// Provides: {"impl_27"}
// Dependencies: {}
impl ErrorCounter { # [doc = " Returns the number of log lines that have been dropped."] # [doc = ""] # [doc = " If the non-blocking writer is not configured in [lossy mode], the error"] # [doc = " count should always be 0."] # [doc = ""] # [doc = " [lossy mode]: NonBlockingBuilder::lossy"] pub fn dropped_lines (& self) -> usize { self . 0 . load (Ordering :: Acquire) } fn incr_saturating (& self) { let mut curr = self . 0 . load (Ordering :: Acquire) ; if curr == usize :: MAX { return ; } loop { let val = curr . saturating_add (1) ; match self . 0 . compare_exchange (curr , val , Ordering :: AcqRel , Ordering :: Acquire) { Ok (_) => return , Err (actual) => curr = actual , } } } }
};
}
