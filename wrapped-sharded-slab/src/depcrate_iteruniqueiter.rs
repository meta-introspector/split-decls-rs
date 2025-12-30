// Generated macro for UniqueIter (struct)
macro_rules! Depcrate_iterUniqueIter {
() => {
// Module: crate::iter
// Provides: {"UniqueIter"}
// Dependencies: {}
# [doc = " An exclusive fused iterator over the items in a [`Slab`](crate::Slab)."] # [must_use = "iterators are lazy and do nothing unless consumed"] # [derive (Debug)] pub struct UniqueIter < 'a , T , C : cfg :: Config > { pub (super) shards : shard :: IterMut < 'a , Option < T > , C > , pub (super) pages : slice :: Iter < 'a , page :: Shared < Option < T > , C > > , pub (super) slots : Option < page :: Iter < 'a , T , C > > , }
};
}
