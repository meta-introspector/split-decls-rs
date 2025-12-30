// Generated macro for impl_292 (impl)
macro_rules! Depcrate_weak_sliceimpl_292 {
() => {
// Module: crate::weak_slice
// Provides: {"impl_292"}
// Dependencies: {}
impl < 'a , T , const N : usize > WeakArrayMut < 'a , T , N > { # [doc = " # Safety"] # [doc = ""] # [doc = " The pointer must be [convertable to a reference](https://doc.rust-lang.org/std/ptr/index.html#pointer-to-reference-conversion)."] pub (crate) unsafe fn from_ptr (ptr : * mut [T ; N]) -> Self { Self { ptr , _marker : PhantomData , } } pub (crate) fn as_slice (& self) -> & 'a [T] { unsafe { core :: slice :: from_raw_parts (self . ptr . cast () , N) } } pub (crate) fn as_mut_slice (& mut self) -> & 'a mut [T] { unsafe { core :: slice :: from_raw_parts_mut (self . ptr . cast () , N) } } pub (crate) fn as_mut_ptr (& mut self) -> * mut [T ; N] { self . ptr } }
};
}
