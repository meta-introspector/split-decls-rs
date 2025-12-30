// Generated macro for Iter (struct)
macro_rules! Depcrate_tree_indexIter {
() => {
// Module: crate::tree_index
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the entries of a [`TreeIndex`]."] # [doc = ""] # [doc = " An [`Iter`] iterates over all the entries that exist during the lifetime of the [`Iter`] in"] # [doc = " monotonically increasing order."] pub struct Iter < 't , 'g , K , V > { root : & 't AtomicShared < Node < K , V > > , leaf_scanner : Option < Scanner < 'g , K , V > > , guard : & 'g Guard , }
};
}
