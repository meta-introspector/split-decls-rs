// Generated macro for Range (struct)
macro_rules! Depcrate_tree_indexRange {
() => {
// Module: crate::tree_index
// Provides: {"Range"}
// Dependencies: {}
# [doc = " An iterator over a sub-range of entries in a [`TreeIndex`]."] pub struct Range < 't , 'g , K , V , Q : ? Sized , R : RangeBounds < Q > > { root : & 't AtomicShared < Node < K , V > > , leaf_scanner : Option < Scanner < 'g , K , V > > , bounds : R , check_lower_bound : bool , check_upper_bound : bool , guard : & 'g Guard , query : PhantomData < fn () -> Q > , }
};
}
