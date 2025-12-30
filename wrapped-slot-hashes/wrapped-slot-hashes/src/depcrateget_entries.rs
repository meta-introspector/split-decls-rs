// Generated macro for get_entries (function)
macro_rules! Depcrateget_entries {
() => {
// Module: crate
// Provides: {"get_entries"}
// Dependencies: {}
pub fn get_entries () -> usize { NUM_ENTRIES . load (Ordering :: Relaxed) }
};
}
