// Generated macro for impl_158 (impl)
macro_rules! Depcrate_map_vecsimpl_158 {
() => {
// Module: crate::map::vecs
// Provides: {"impl_158"}
// Dependencies: {}
impl < T > ZeroVecLike < T > for ZeroSlice < T > where T : AsULE + Copy , { type GetType = T :: ULE ; type SliceVariant = ZeroSlice < T > ; fn zvl_new_borrowed () -> & 'static Self :: SliceVariant { ZeroSlice :: < T > :: new_empty () } fn zvl_binary_search (& self , k : & T) -> Result < usize , usize > where T : Ord , { ZeroSlice :: binary_search (self , k) } fn zvl_binary_search_in_range (& self , k : & T , range : Range < usize >) -> Option < Result < usize , usize > > where T : Ord , { let subslice = self . get_subslice (range) ? ; Some (ZeroSlice :: binary_search (subslice , k)) } fn zvl_binary_search_by (& self , mut predicate : impl FnMut (& T) -> Ordering ,) -> Result < usize , usize > { ZeroSlice :: binary_search_by (self , | probe | predicate (& probe)) } fn zvl_binary_search_in_range_by (& self , mut predicate : impl FnMut (& T) -> Ordering , range : Range < usize > ,) -> Option < Result < usize , usize > > { let subslice = self . get_subslice (range) ? ; Some (ZeroSlice :: binary_search_by (subslice , | probe | { predicate (& probe) })) } fn zvl_get (& self , index : usize) -> Option < & T :: ULE > { self . get_ule_ref (index) } fn zvl_len (& self) -> usize { ZeroSlice :: len (self) } fn zvl_as_borrowed (& self) -> & ZeroSlice < T > { self } # [inline] fn zvl_get_as_t < R > (g : & Self :: GetType , f : impl FnOnce (& T) -> R) -> R { f (& T :: from_unaligned (* g)) } }
};
}
