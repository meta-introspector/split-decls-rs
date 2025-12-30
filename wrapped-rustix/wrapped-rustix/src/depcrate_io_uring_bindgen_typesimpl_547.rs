// Generated macro for impl_547 (impl)
macro_rules! Depcrate_io_uring_bindgen_typesimpl_547 {
() => {
// Module: crate::io_uring::bindgen_types
// Provides: {"impl_547"}
// Dependencies: {}
# [allow (missing_docs)] impl < T > IncompleteArrayField < T > { # [inline] pub const fn new () -> Self { Self (:: core :: marker :: PhantomData , []) } # [inline] pub fn as_ptr (& self) -> * const T { as_ptr (self) . cast :: < T > () } # [inline] pub fn as_mut_ptr (& mut self) -> * mut T { as_mut_ptr (self) . cast :: < T > () } # [inline] pub unsafe fn as_slice (& self , len : usize) -> & [T] { :: core :: slice :: from_raw_parts (self . as_ptr () , len) } # [inline] pub unsafe fn as_mut_slice (& mut self , len : usize) -> & mut [T] { :: core :: slice :: from_raw_parts_mut (self . as_mut_ptr () , len) } }
};
}
