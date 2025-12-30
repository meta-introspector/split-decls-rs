// Generated macro for impl_326 (impl)
macro_rules! Depcrate_varzerovec_componentsimpl_326 {
() => {
// Module: crate::varzerovec::components
// Provides: {"impl_326"}
// Dependencies: {}
unsafe impl IntegerULE for u8 { const TOO_LARGE_ERROR : & 'static str = "Attempted to build VarZeroVec out of elements that \
                                     cumulatively are larger than a u8 in size" ; const SIZE : usize = mem :: size_of :: < Self > () ; const MAX_VALUE : u32 = u8 :: MAX as u32 ; # [inline] fn iule_to_usize (self) -> usize { self as usize } # [inline] fn iule_from_usize (u : usize) -> Option < Self > { u8 :: try_from (u) . ok () } # [inline] # [cfg (feature = "alloc")] fn iule_from_bytes_unchecked_mut (bytes : & mut [u8]) -> & mut [Self] { bytes } }
};
}
