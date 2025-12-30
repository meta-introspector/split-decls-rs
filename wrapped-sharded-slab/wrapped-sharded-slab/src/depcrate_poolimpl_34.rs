// Generated macro for impl_34 (impl)
macro_rules! Depcrate_poolimpl_34 {
() => {
// Module: crate::pool
// Provides: {"impl_34"}
// Dependencies: {}
impl < T , C > Drop for OwnedRef < T , C > where T : Clear + Default , C : cfg :: Config , { fn drop (& mut self) { test_println ! ("drop OwnedRef: try clearing data") ; let should_clear = unsafe { self . inner . release () } ; if should_clear { let shard_idx = Tid :: < C > :: from_packed (self . key) ; test_println ! ("-> shard={:?}" , shard_idx) ; if let Some (shard) = self . pool . shards . get (shard_idx . as_usize ()) { shard . clear_after_release (self . key) ; } else { test_println ! ("-> shard={:?} does not exist! THIS IS A BUG" , shard_idx) ; debug_assert ! (std :: thread :: panicking () , "[internal error] tried to drop an `OwnedRef` to a slot on a shard that never existed!") ; } } } }
};
}
