// Generated macro for impl_342 (impl)
macro_rules! Depcrate_pointer_innerimpl_342 {
() => {
// Module: crate::pointer::inner
// Provides: {"impl_342"}
// Dependencies: {}
impl < 'a , T , const N : usize > PtrInner < 'a , [T ; N] > { # [doc = " Casts this pointer-to-array into a slice."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers may assume that the returned `PtrInner` references the same"] # [doc = " address and length as `self`."] # [allow (clippy :: wrong_self_convention)] pub (crate) fn as_slice (self) -> PtrInner < 'a , [T] > { let start = self . as_non_null () . cast :: < T > () . as_ptr () ; let slice = core :: ptr :: slice_from_raw_parts_mut (start , N) ; let slice = unsafe { NonNull :: new_unchecked (slice) } ; unsafe { PtrInner :: new (slice) } } }
};
}
