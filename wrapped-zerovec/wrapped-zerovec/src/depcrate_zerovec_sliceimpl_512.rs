// Generated macro for impl_512 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_512 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_512"}
// Dependencies: {}
impl < T > ZeroSlice < T > where T : AsULE , { # [doc = " Binary searches a sorted `ZeroVec<T>` based on a given predicate. For more information, see"] # [doc = " the primitive function [`binary_search_by`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use zerovec::ZeroVec;"] # [doc = ""] # [doc = " let bytes: &[u8] = &[0xD3, 0x00, 0x19, 0x01, 0xA5, 0x01, 0xCD, 0x80];"] # [doc = " let zerovec: ZeroVec<u16> ="] # [doc = "     ZeroVec::parse_bytes(bytes).expect(\"infallible\");"] # [doc = ""] # [doc = " assert_eq!(zerovec.binary_search_by(|x| x.cmp(&281)), Ok(1));"] # [doc = " assert_eq!(zerovec.binary_search_by(|x| x.cmp(&282)), Err(2));"] # [doc = " ```"] # [doc = ""] # [doc = " [`binary_search_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search_by"] # [inline] pub fn binary_search_by (& self , mut predicate : impl FnMut (T) -> Ordering ,) -> Result < usize , usize > { self . as_ule_slice () . binary_search_by (| probe | predicate (T :: from_unaligned (* probe))) } }
};
}
