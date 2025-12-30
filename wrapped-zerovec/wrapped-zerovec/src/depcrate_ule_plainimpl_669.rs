// Generated macro for impl_669 (impl)
macro_rules! Depcrate_ule_plainimpl_669 {
() => {
// Module: crate::ule::plain
// Provides: {"impl_669"}
// Dependencies: {}
impl < const N : usize > RawBytesULE < N > { # [inline] pub fn as_bytes (& self) -> & [u8] { & self . 0 } # [inline] pub fn from_bytes_unchecked_mut (bytes : & mut [u8]) -> & mut [Self] { let data = bytes . as_mut_ptr () ; let len = bytes . len () / N ; unsafe { core :: slice :: from_raw_parts_mut (data as * mut Self , len) } } }
};
}
