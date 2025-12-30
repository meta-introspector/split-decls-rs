// Generated macro for impl_182 (impl)
macro_rules! Depcrate_shardimpl_182 {
() => {
// Module: crate::shard
// Provides: {"impl_182"}
// Dependencies: {}
impl < T , C : cfg :: Config > Drop for Array < T , C > { fn drop (& mut self) { let max = self . max . load (Acquire) ; for shard in & self . shards [0 ..= max] { let ptr = shard . 0 . load (Acquire) ; if ptr . is_null () { continue ; } let shard = unsafe { Box :: from_raw (ptr) } ; drop (shard) } } }
};
}
