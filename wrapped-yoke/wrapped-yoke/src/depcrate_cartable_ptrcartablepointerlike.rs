// Generated macro for CartablePointerLike (trait)
macro_rules! Depcrate_cartable_ptrCartablePointerLike {
() => {
// Module: crate::cartable_ptr
// Provides: {"CartablePointerLike"}
// Dependencies: {}
# [doc = " An object fully representable by a non-null pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementer safety:"] # [doc = ""] # [doc = " 1. `into_raw` transfers ownership of the values referenced by StableDeref to the caller,"] # [doc = "    if there is ownership to transfer"] # [doc = " 2. `drop_raw` returns ownership back to the impl, if there is ownership to transfer"] # [doc = ""] # [doc = " Note: if `into_raw` returns the sentinel pointer, memory leaks may occur, but this will not"] # [doc = " lead to undefined behaviour."] # [doc = ""] # [doc = " Note: the pointer `NonNull<Self::Raw>` may or may not be aligned and it should never"] # [doc = " be dereferenced. Rust allows unaligned pointers; see [`std::ptr::read_unaligned`]."] pub unsafe trait CartablePointerLike : StableDeref + Sealed { # [doc = " The raw type used for [`Self::into_raw`] and [`Self::drop_raw`]."] # [doc (hidden)] type Raw ; # [doc = " Converts this pointer-like into a pointer."] # [doc (hidden)] fn into_raw (self) -> NonNull < Self :: Raw > ; # [doc = " Drops any memory associated with this pointer-like."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Caller safety:"] # [doc = ""] # [doc = " 1. The pointer MUST have been returned by this impl's `into_raw`."] # [doc = " 2. The pointer MUST NOT be dangling."] # [doc (hidden)] unsafe fn drop_raw (pointer : NonNull < Self :: Raw >) ; }
};
}
