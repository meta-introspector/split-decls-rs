// Generated macro for IntRange (struct)
macro_rules! Depcrate_constructorIntRange {
() => {
// Module: crate::constructor
// Provides: {"IntRange"}
// Dependencies: {}
# [doc = " An exclusive interval, used for precise integer exhaustiveness checking. `IntRange`s always"] # [doc = " store a contiguous range."] # [doc = ""] # [doc = " `IntRange` is never used to encode an empty range or a \"range\" that wraps around the (offset)"] # [doc = " space: i.e., `range.lo < range.hi`."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct IntRange { pub lo : MaybeInfiniteInt , pub hi : MaybeInfiniteInt , }
};
}
