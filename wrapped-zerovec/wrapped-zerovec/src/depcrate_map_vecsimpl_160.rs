// Generated macro for impl_160 (impl)
macro_rules! Depcrate_map_vecsimpl_160 {
() => {
// Module: crate::map::vecs
// Provides: {"impl_160"}
// Dependencies: {}
impl < 'a , T , F > ZeroVecLike < T > for VarZeroVec < 'a , T , F > where T : VarULE , T : ? Sized , F : VarZeroVecFormat , { type GetType = T ; type SliceVariant = VarZeroSlice < T , F > ; fn zvl_new_borrowed () -> & 'static Self :: SliceVariant { VarZeroSlice :: < T , F > :: new_empty () } fn zvl_binary_search (& self , k : & T) -> Result < usize , usize > where T : Ord , { self . binary_search (k) } fn zvl_binary_search_in_range (& self , k : & T , range : Range < usize >) -> Option < Result < usize , usize > > where T : Ord , { self . binary_search_in_range (k , range) } fn zvl_binary_search_by (& self , predicate : impl FnMut (& T) -> Ordering) -> Result < usize , usize > { self . binary_search_by (predicate) } fn zvl_binary_search_in_range_by (& self , predicate : impl FnMut (& T) -> Ordering , range : Range < usize > ,) -> Option < Result < usize , usize > > { self . binary_search_in_range_by (predicate , range) } fn zvl_get (& self , index : usize) -> Option < & T > { self . get (index) } fn zvl_len (& self) -> usize { self . len () } fn zvl_as_borrowed (& self) -> & VarZeroSlice < T , F > { self . as_slice () } # [inline] fn zvl_get_as_t < R > (g : & Self :: GetType , f : impl FnOnce (& T) -> R) -> R { f (g) } }
};
}
