// Generated macro for impl_94 (impl)
macro_rules! Depcrateimpl_94 {
() => {
// Module: crate
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'a , T > Drop for Drain < 'a , T > { fn drop (& mut self) { for _ in self . by_ref () { } unsafe { let vec = self . vec . as_mut () ; if ! vec . is_singleton () { let old_len = vec . len () ; let start = vec . data_raw () . add (old_len) ; let end = vec . data_raw () . add (self . end) ; ptr :: copy (end , start , self . tail) ; vec . set_len_non_singleton (old_len + self . tail) ; } } } }
};
}
