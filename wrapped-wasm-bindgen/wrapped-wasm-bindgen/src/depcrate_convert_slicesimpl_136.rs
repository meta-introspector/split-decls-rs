// Generated macro for impl_136 (impl)
macro_rules! Depcrate_convert_slicesimpl_136 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_136"}
// Dependencies: {}
impl < T > Drop for MutSlice < T > { fn drop (& mut self) { let byte_slice = unsafe { core :: slice :: from_raw_parts (self . contents . as_ptr () as * const u8 , self . contents . len () * mem :: size_of :: < T > () ,) } ; __wbindgen_copy_to_typed_array (byte_slice , & self . js) ; } }
};
}
