// Generated macro for impl_511 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_511 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_511"}
// Dependencies: {}
impl < T > ZeroSlice < T > where T : AsULE + Ord , { # [doc = " Binary searches a sorted `ZeroVec<T>` for the given element. For more information, see"] # [doc = " the primitive function [`binary_search`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use zerovec::ZeroVec;"] # [doc = ""] # [doc = " let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];"] # [doc = " let zerovec: ZeroVec<u16> ="] # [doc = "     ZeroVec::parse_bytes(bytes).expect(\"infallible\");"] # [doc = ""] # [doc = " assert_eq!(zerovec.binary_search(&281), Ok(1));"] # [doc = " assert_eq!(zerovec.binary_search(&282), Err(2));"] # [doc = " ```"] # [doc = ""] # [doc = " [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search"] # [inline] pub fn binary_search (& self , x : & T) -> Result < usize , usize > { self . as_ule_slice () . binary_search_by (| probe | T :: from_unaligned (* probe) . cmp (x)) } }
};
}
