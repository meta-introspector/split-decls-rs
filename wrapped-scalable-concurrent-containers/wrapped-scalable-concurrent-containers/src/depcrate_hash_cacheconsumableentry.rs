// Generated macro for ConsumableEntry (struct)
macro_rules! Depcrate_hash_cacheConsumableEntry {
() => {
// Module: crate::hash_cache
// Provides: {"ConsumableEntry"}
// Dependencies: {}
# [doc = " [`ConsumableEntry`] is a view into an occupied entry in a [`HashCache`] when iterating over"] # [doc = " entries in it."] pub struct ConsumableEntry < 'b , 'g : 'b , K , V > { # [doc = " Holds an exclusive lock on the entry bucket."] locked_bucket : & 'b mut LockedBucket < K , V , DoublyLinkedList , CACHE > , # [doc = " Pointer to the entry."] entry_ptr : & 'b mut EntryPtr < 'g , K , V , CACHE > , # [doc = " Probes removal."] remove_probe : & 'b mut bool , # [doc = " Associated [`Guard`]."] guard : & 'g Guard , }
};
}
