// Generated macro for impl_560 (impl)
macro_rules! Depcrate_zerovecimpl_560 {
() => {
// Module: crate::zerovec
// Provides: {"impl_560"}
// Dependencies: {}
impl < 'a , T > ZeroVec < 'a , T > where T : AsULE , { # [doc = " Creates a `ZeroVec<T>` from a `&[T]` by allocating memory."] # [doc = ""] # [doc = " This function results in an `Owned` instance of `ZeroVec<T>`."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use zerovec::ZeroVec;"] # [doc = ""] # [doc = " // The little-endian bytes correspond to the numbers on the following line."] # [doc = " let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x01];"] # [doc = " let nums: &[u16] = &[211, 281, 421, 461];"] # [doc = ""] # [doc = " let zerovec = ZeroVec::alloc_from_slice(nums);"] # [doc = ""] # [doc = " assert!(zerovec.is_owned());"] # [doc = " assert_eq!(bytes, zerovec.as_bytes());"] # [doc = " ```"] # [inline] # [cfg (feature = "alloc")] pub fn alloc_from_slice (other : & [T]) -> Self { Self :: new_owned (other . iter () . copied () . map (T :: to_unaligned) . collect ()) } # [doc = " Creates a `Vec<T>` from a `ZeroVec<T>`."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use zerovec::ZeroVec;"] # [doc = ""] # [doc = " let nums: &[u16] = &[211, 281, 421, 461];"] # [doc = " let vec: Vec<u16> = ZeroVec::alloc_from_slice(nums).to_vec();"] # [doc = ""] # [doc = " assert_eq!(nums, vec.as_slice());"] # [doc = " ```"] # [inline] # [cfg (feature = "alloc")] pub fn to_vec (& self) -> Vec < T > { self . iter () . collect () } }
};
}
