// Generated macro for test_fn (function)
macro_rules! Depcratetest_fn {
() => {
// Module: crate
// Provides: {"test_fn"}
// Dependencies: {}
pub fn test_fn (key : & str , count : usize) { init () ; let local_locks = LOCKS . get_or_init (HashMap :: new) ; let entry = local_locks . entry (key . to_string ()) . or_insert (AtomicUsize :: new (0)) ; let local_lock = entry . get () ; info ! ("(non-fs) Start {}" , count) ; local_lock . store (count , Ordering :: Relaxed) ; thread :: sleep (Duration :: from_millis (1000 * (count as u64))) ; info ! ("(non-fs) End {}" , count) ; assert_eq ! (local_lock . load (Ordering :: Relaxed) , count) ; }
};
}
