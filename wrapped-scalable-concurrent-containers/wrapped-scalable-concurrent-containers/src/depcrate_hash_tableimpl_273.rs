// Generated macro for impl_273 (impl)
macro_rules! Depcrate_hash_tableimpl_273 {
() => {
// Module: crate::hash_table
// Provides: {"impl_273"}
// Dependencies: {}
impl < K , V , L : LruList , const TYPE : char > LockedBucket < K , V , L , TYPE > { # [doc = " Returns a reference to the [`BucketArray`] that contains this [`LockedBucket`]."] # [inline] pub (crate) const fn bucket_array (& self) -> & BucketArray < K , V , L , TYPE > { unsafe { self . bucket_array . as_ref () } } # [doc = " Gets a mutable reference to the entry."] # [inline] pub (crate) fn entry < 'b , 'g : 'b > (& 'b self , entry_ptr : & 'b EntryPtr < 'g , K , V , TYPE > ,) -> & 'b (K , V) { entry_ptr . get (self . data_block) } # [doc = " Gets a mutable reference to the entry."] # [inline] pub (crate) fn entry_mut < 'b , 'g : 'b > (& 'b mut self , entry_ptr : & 'b mut EntryPtr < 'g , K , V , TYPE > ,) -> & 'b mut (K , V) { entry_ptr . get_mut (self . data_block , & self . writer) } # [doc = " Inserts a new entry with the supplied constructor function."] # [inline] pub (crate) fn insert < 'g > (& self , hash : u64 , entry : (K , V) , guard : & 'g Guard ,) -> EntryPtr < 'g , K , V , TYPE > { if TYPE == INDEX { self . writer . try_drop_unreachable_entries (self . data_block , guard) ; } match self . writer . insert (self . data_block , hash , entry , guard) { Ok (entry_ptr) => entry_ptr , Err (entry) => self . writer . insert_overflow (hash , entry , guard) , } } }
};
}
