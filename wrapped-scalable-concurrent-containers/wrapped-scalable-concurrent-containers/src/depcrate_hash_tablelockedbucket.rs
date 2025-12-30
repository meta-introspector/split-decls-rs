// Generated macro for LockedBucket (struct)
macro_rules! Depcrate_hash_tableLockedBucket {
() => {
// Module: crate::hash_table
// Provides: {"LockedBucket"}
// Dependencies: {}
# [doc = " [`LockedBucket`] has exclusive access to a [`Bucket`]."] # [derive (Debug)] pub (crate) struct LockedBucket < K , V , L : LruList , const TYPE : char > { # [doc = " Holds an exclusive lock on the [`Bucket`]."] pub writer : Writer < K , V , L , TYPE > , # [doc = " Corresponding [`DataBlock`]."] pub data_block : NonNull < DataBlock < K , V , BUCKET_LEN > > , # [doc = " The index of the [`Bucket`] within the [`BucketArray`]."] pub bucket_index : usize , # [doc = " Corresponding [`BucketArray`]."] # [doc = ""] # [doc = " The [`BucketArray`] is not dropped as long as it holds an exclusive lock on the [`Bucket`]."] pub bucket_array : NonNull < BucketArray < K , V , L , TYPE > > , }
};
}
