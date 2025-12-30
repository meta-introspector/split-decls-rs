// Generated macro for impl_275 (impl)
macro_rules! Depcrate_hash_tableimpl_275 {
() => {
// Module: crate::hash_table
// Provides: {"impl_275"}
// Dependencies: {}
impl < K , V , L : LruList , const TYPE : char > Deref for LockedBucket < K , V , L , TYPE > { type Target = Bucket < K , V , L , TYPE > ; # [inline] fn deref (& self) -> & Self :: Target { & self . writer } }
};
}
