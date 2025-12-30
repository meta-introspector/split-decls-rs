// Generated macro for impl_220 (impl)
macro_rules! Depcrate_tree_indeximpl_220 {
() => {
// Module: crate::tree_index
// Provides: {"impl_220"}
// Dependencies: {}
impl < 't , 'g , K , V , Q : ? Sized , R : RangeBounds < Q > > Range < 't , 'g , K , V , Q , R > { # [inline] fn new (root : & 't AtomicShared < Node < K , V > > , range : R , guard : & 'g Guard ,) -> Range < 't , 'g , K , V , Q , R > { Range :: < 't , 'g , K , V , Q , R > { root , leaf_scanner : None , bounds : range , check_lower_bound : true , check_upper_bound : false , guard , query : PhantomData , } } }
};
}
