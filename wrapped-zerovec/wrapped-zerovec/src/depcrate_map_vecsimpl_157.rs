// Generated macro for impl_157 (impl)
macro_rules! Depcrate_map_vecsimpl_157 {
() => {
// Module: crate::map::vecs
// Provides: {"impl_157"}
// Dependencies: {}
impl < 'a , T > ZeroVecLike < T > for ZeroVec < 'a , T > where T : 'a + AsULE + Copy , { type GetType = T :: ULE ; type SliceVariant = ZeroSlice < T > ; fn zvl_new_borrowed () -> & 'static Self :: SliceVariant { ZeroSlice :: < T > :: new_empty () } fn zvl_binary_search (& self , k : & T) -> Result < usize , usize > where T : Ord , { ZeroSlice :: binary_search (self , k) } fn zvl_binary_search_in_range (& self , k : & T , range : Range < usize >) -> Option < Result < usize , usize > > where T : Ord , { let zs : & ZeroSlice < T > = self ; zs . zvl_binary_search_in_range (k , range) } fn zvl_binary_search_by (& self , mut predicate : impl FnMut (& T) -> Ordering ,) -> Result < usize , usize > { ZeroSlice :: binary_search_by (self , | probe | predicate (& probe)) } fn zvl_binary_search_in_range_by (& self , predicate : impl FnMut (& T) -> Ordering , range : Range < usize > ,) -> Option < Result < usize , usize > > { let zs : & ZeroSlice < T > = self ; zs . zvl_binary_search_in_range_by (predicate , range) } fn zvl_get (& self , index : usize) -> Option < & T :: ULE > { self . get_ule_ref (index) } fn zvl_len (& self) -> usize { ZeroSlice :: len (self) } fn zvl_as_borrowed (& self) -> & ZeroSlice < T > { self } # [inline] fn zvl_get_as_t < R > (g : & Self :: GetType , f : impl FnOnce (& T) -> R) -> R { f (& T :: from_unaligned (* g)) } }
};
}
