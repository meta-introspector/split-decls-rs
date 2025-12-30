// Generated macro for impl_328 (impl)
macro_rules! Depcrate_varzerovec_componentsimpl_328 {
() => {
// Module: crate::varzerovec::components
// Provides: {"impl_328"}
// Dependencies: {}
unsafe impl IntegerULE for RawBytesULE < 4 > { const TOO_LARGE_ERROR : & 'static str = "Attempted to build VarZeroVec out of elements that \
                                     cumulatively are larger than a u32 in size" ; const SIZE : usize = mem :: size_of :: < Self > () ; const MAX_VALUE : u32 = u32 :: MAX ; # [inline] fn iule_to_usize (self) -> usize { self . as_unsigned_int () as usize } # [inline] fn iule_from_usize (u : usize) -> Option < Self > { u32 :: try_from (u) . ok () . map (u32 :: to_unaligned) } # [inline] # [cfg (feature = "alloc")] fn iule_from_bytes_unchecked_mut (bytes : & mut [u8]) -> & mut [Self] { Self :: from_bytes_unchecked_mut (bytes) } }
};
}
