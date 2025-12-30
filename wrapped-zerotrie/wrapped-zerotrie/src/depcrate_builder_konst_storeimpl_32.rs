// Generated macro for impl_32 (impl)
macro_rules! Depcrate_builder_konst_storeimpl_32 {
() => {
// Module: crate::builder::konst::store
// Provides: {"impl_32"}
// Dependencies: {}
impl < const N : usize , T > ConstArrayBuilder < N , T > { # [doc = " Creates a new, empty builder of the given size. `cursor` indicates where in the"] # [doc = " array new elements will be inserted first. Since we use a lot of prepend operations,"] # [doc = " it is common to set `cursor` to `N`."] pub const fn new_empty (full_array : [T ; N] , cursor : usize) -> Self { assert ! (cursor <= N) ; Self { full_array , start : cursor , limit : cursor , } } # [doc = " Creates a new builder with some initial content in `[start, limit)`."] pub const fn from_manual_slice (full_array : [T ; N] , start : usize , limit : usize) -> Self { assert ! (start <= limit) ; assert ! (limit <= N) ; Self { full_array , start , limit , } } # [doc = " Returns the number of initialized elements in the builder."] pub const fn len (& self) -> usize { self . limit - self . start } # [doc = " Whether there are no initialized elements in the builder."] # [allow (dead_code)] pub const fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Returns the initialized elements as a [`ConstSlice`]."] pub const fn as_const_slice (& self) -> ConstSlice < '_ , T > { ConstSlice :: from_manual_slice (& self . full_array , self . start , self . limit) } # [doc = " Non-const function that returns a slice of the initialized elements."] # [cfg (any (test , feature = "alloc"))] pub fn as_slice (& self) -> & [T] { & self . full_array [self . start .. self . limit] } }
};
}
