// Generated macro for impl_9 (impl)
macro_rules! Depcrate_cacheimpl_9 {
() => {
// Module: crate::cache
// Provides: {"impl_9"}
// Dependencies: {}
impl < Key , Value > Cache < Key , Value > { # [doc = " Actually frees the underlying memory in contrast to what stdlib containers do on `clear`"] pub fn clear (& self) { * self . hashmap . borrow_mut () = Default :: default () ; } }
};
}
