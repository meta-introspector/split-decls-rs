// Generated macro for impl_128 (impl)
macro_rules! Depcrate_iterimpl_128 {
() => {
// Module: crate::iter
// Provides: {"impl_128"}
// Dependencies: {}
impl < 'lookup , Old , New , T > ChangesIter < 'lookup , Old , New , T > where Old : Index < usize , Output = T > + ? Sized , New : Index < usize , Output = T > + ? Sized , { pub (crate) fn new (old : & 'lookup Old , new : & 'lookup New , op : DiffOp) -> Self { let (tag , old_range , new_range) = op . as_tag_tuple () ; let old_index = old_range . start ; let new_index = new_range . start ; let old_i = old_range . start ; let new_i = new_range . start ; ChangesIter { old , new , old_range , new_range , old_index , new_index , old_i , new_i , tag , _marker : PhantomData , } } }
};
}
