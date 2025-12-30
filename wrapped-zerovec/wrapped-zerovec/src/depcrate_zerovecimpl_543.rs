// Generated macro for impl_543 (impl)
macro_rules! Depcrate_zerovecimpl_543 {
() => {
// Module: crate::zerovec
// Provides: {"impl_543"}
// Dependencies: {}
impl < U > EyepatchHackVector < U > { # [inline] unsafe fn as_arbitrary_slice < 'a > (& self) -> & 'a [U] { self . buf . as_ref () } # [inline] const fn as_slice < 'a > (& 'a self) -> & 'a [U] { unsafe { & * (self . buf . as_ptr () as * const [U]) } } # [doc = " Return this type as a vector"] # [doc = ""] # [doc = " Data MUST be known to be owned beforehand"] # [doc = ""] # [doc = " Because this borrows self, this is effectively creating two owners to the same"] # [doc = " data, make sure that `self` is cleaned up after this"] # [doc = ""] # [doc = " (this does not simply take `self` since then it wouldn't be usable from the Drop impl)"] # [cfg (feature = "alloc")] unsafe fn get_vec (& self) -> Vec < U > { debug_assert ! (self . capacity != 0) ; let slice : & [U] = self . as_slice () ; let len = slice . len () ; Vec :: from_raw_parts (self . buf . as_ptr () as * mut U , len , self . capacity) } fn truncate (& mut self , max : usize) { self . buf = unsafe { NonNull :: new_unchecked (core :: ptr :: slice_from_raw_parts_mut (self . buf . as_mut () . as_mut_ptr () , core :: cmp :: min (max , self . buf . as_ref () . len ()) ,)) } ; } }
};
}
