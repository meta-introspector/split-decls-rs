// Generated macro for impl_670 (impl)
macro_rules! Depcrate_zalsaimpl_670 {
() => {
// Module: crate::zalsa
// Provides: {"impl_670"}
// Dependencies: {}
impl IngredientIndex { # [doc = " The maximum supported ingredient index."] # [doc = ""] # [doc = " This reserves one bit for an optional tag."] const MAX_INDEX : u32 = 0x7FFF_FFFF ; # [doc = " Create an ingredient index from a `u32`."] pub (crate) fn new (v : u32) -> Self { assert ! (v <= Self :: MAX_INDEX) ; Self (v) } # [doc = " Create an ingredient index from a `u32`, without performing validating"] # [doc = " that the index is valid."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The index must be less than or equal to `IngredientIndex::MAX_INDEX`."] pub (crate) unsafe fn new_unchecked (v : u32) -> Self { Self (v) } # [doc = " Convert the ingredient index back into a `u32`."] pub (crate) fn as_u32 (self) -> u32 { self . 0 } pub fn successor (self , index : usize) -> Self { IngredientIndex (self . 0 + 1 + index as u32) } # [doc = " Returns a new `IngredientIndex` with the tag bit set to the provided value."] pub (crate) fn with_tag (mut self , tag : bool) -> IngredientIndex { self . 0 &= Self :: MAX_INDEX ; self . 0 |= (tag as u32) << 31 ; self } # [doc = " Returns the value of the tag bit."] pub (crate) fn tag (self) -> bool { self . 0 & ! Self :: MAX_INDEX != 0 } }
};
}
