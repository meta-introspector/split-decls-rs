// Generated macro for set_entries_for_tests_only (function)
macro_rules! Depcrateset_entries_for_tests_only {
() => {
// Module: crate
// Provides: {"set_entries_for_tests_only"}
// Dependencies: {}
pub fn set_entries_for_tests_only (entries : usize) { NUM_ENTRIES . store (entries , Ordering :: Relaxed) ; }
};
}
