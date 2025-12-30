// Generated macro for ConsumableEntry (struct)
macro_rules! Depcrate_hash_mapConsumableEntry {
() => {
// Module: crate::hash_map
// Provides: {"ConsumableEntry"}
// Dependencies: {}
# [doc = " [`ConsumableEntry`] is a view into an occupied entry in a [`HashMap`] when iterating over"] # [doc = " entries in it."] pub struct ConsumableEntry < 'b , 'g : 'b , K , V > { # [doc = " Holds an exclusive lock on the entry bucket."] locked_bucket : & 'b mut LockedBucket < K , V , () , MAP > , # [doc = " Pointer to the entry."] entry_ptr : & 'b mut EntryPtr < 'g , K , V , MAP > , # [doc = " Probes removal."] remove_probe : & 'b mut bool , # [doc = " Associated [`Guard`]."] guard : & 'g Guard , }
};
}
