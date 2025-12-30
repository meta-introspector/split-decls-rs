// Generated macro for Iter (struct)
macro_rules! Depcrate_hash_indexIter {
() => {
// Module: crate::hash_index
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the entries of a [`HashIndex`]."] # [doc = ""] # [doc = " An [`Iter`] iterates over all the entries that survive the [`Iter`]."] pub struct Iter < 'h , K , V , H = RandomState > where H : BuildHasher , { hashindex : & 'h HashIndex < K , V , H > , bucket_array : Option < & 'h BucketArray < K , V , () , INDEX > > , index : usize , bucket : Option < & 'h Bucket < K , V , () , INDEX > > , entry_ptr : EntryPtr < 'h , K , V , INDEX > , guard : & 'h Guard , }
};
}
