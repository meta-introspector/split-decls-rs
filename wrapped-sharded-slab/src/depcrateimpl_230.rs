// Generated macro for impl_230 (impl)
macro_rules! Depcrateimpl_230 {
() => {
// Module: crate
// Provides: {"impl_230"}
// Dependencies: {}
impl < T , C > Drop for OwnedEntry < T , C > where C : cfg :: Config , { fn drop (& mut self) { test_println ! ("drop OwnedEntry: try clearing data") ; let should_clear = unsafe { self . inner . release () } ; if should_clear { let shard_idx = Tid :: < C > :: from_packed (self . key) ; test_println ! ("-> shard={:?}" , shard_idx) ; if let Some (shard) = self . slab . shards . get (shard_idx . as_usize ()) { shard . clear_after_release (self . key) } else { test_println ! ("-> shard={:?} does not exist! THIS IS A BUG" , shard_idx) ; debug_assert ! (std :: thread :: panicking () , "[internal error] tried to drop an `OwnedEntry` to a slot on a shard that never existed!") ; } } } }
};
}
