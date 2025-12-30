// Generated macro for LengthHint (struct)
macro_rules! DepcrateLengthHint {
() => {
// Module: crate
// Provides: {"LengthHint"}
// Dependencies: {}
# [doc = " A hint to help consumers of `Writeable` pre-allocate bytes before they call"] # [doc = " [`write_to`](Writeable::write_to)."] # [doc = ""] # [doc = " This behaves like `Iterator::size_hint`: it is a tuple where the first element is the"] # [doc = " lower bound, and the second element is the upper bound. If the upper bound is `None`"] # [doc = " either there is no known upper bound, or the upper bound is larger than `usize`."] # [doc = ""] # [doc = " `LengthHint` implements std`::ops::{Add, Mul}` and similar traits for easy composition."] # [doc = " During computation, the lower bound will saturate at `usize::MAX`, while the upper"] # [doc = " bound will become `None` if `usize::MAX` is exceeded."] # [derive (Debug , PartialEq , Eq , Copy , Clone)] # [non_exhaustive] pub struct LengthHint (pub usize , pub Option < usize >) ;
};
}
