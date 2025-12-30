// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl AtomicInterval { # [doc = " true if 'interval_time_ms' has elapsed since last time we returned true as long as it has been 'interval_time_ms' since this struct was created"] # [inline (always)] pub fn should_update (& self , interval_time_ms : u64) -> bool { self . should_update_ext (interval_time_ms , true) } # [doc = " a primary use case is periodic metric reporting, potentially from different threads"] # [doc = " true if 'interval_time_ms' has elapsed since last time we returned true"] # [doc = " except, if skip_first=false, false until 'interval_time_ms' has elapsed since this struct was created"] # [inline (always)] pub fn should_update_ext (& self , interval_time_ms : u64 , skip_first : bool) -> bool { let now = timestamp () ; let last = self . last_update . load (Ordering :: Relaxed) ; now . saturating_sub (last) > interval_time_ms && self . last_update . compare_exchange (last , now , Ordering :: Relaxed , Ordering :: Relaxed) == Ok (last) && ! (skip_first && last == 0) } # [doc = " return ms elapsed since the last time the time was set"] pub fn elapsed_ms (& self) -> u64 { let now = timestamp () ; let last = self . last_update . load (Ordering :: Relaxed) ; now . saturating_sub (last) } # [doc = " return ms until the interval_time will have elapsed"] pub fn remaining_until_next_interval (& self , interval_time : u64) -> u64 { interval_time . saturating_sub (self . elapsed_ms ()) } }
};
}
