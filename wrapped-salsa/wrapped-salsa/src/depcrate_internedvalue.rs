// Generated macro for Value (struct)
macro_rules! Depcrate_internedValue {
() => {
// Module: crate::interned
// Provides: {"Value"}
// Dependencies: {}
# [doc = " Struct storing the interned fields."] pub struct Value < C > where C : Configuration , { # [doc = " The index of the shard containing this value."] shard : u16 , # [doc = " An intrusive linked list for LRU."] link : LinkedListLink , # [doc = " The interned fields for this value."] # [doc = ""] # [doc = " These are valid for read-only access as long as the lock is held"] # [doc = " or the value has been validated in the current revision."] fields : UnsafeCell < C :: Fields < 'static > > , # [doc = " Memos attached to this interned value."] # [doc = ""] # [doc = " This is valid for read-only access as long as the lock is held"] # [doc = " or the value has been validated in the current revision."] memos : UnsafeCell < MemoTable > , # [doc = " Data that can only be accessed while holding the lock for the"] # [doc = " `key_map` shard containing the value ID."] shared : UnsafeCell < ValueShared > , }
};
}
