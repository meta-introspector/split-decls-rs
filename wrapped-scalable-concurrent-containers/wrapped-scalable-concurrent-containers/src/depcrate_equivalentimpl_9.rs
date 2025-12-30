// Generated macro for impl_9 (impl)
macro_rules! Depcrate_equivalentimpl_9 {
() => {
// Module: crate::equivalent
// Provides: {"impl_9"}
// Dependencies: {}
impl < Q : ? Sized , K : ? Sized > Comparable < K > for Q where Q : Ord , K : Borrow < Q > , { # [inline] fn compare (& self , key : & K) -> Ordering { Ord :: cmp (self , key . borrow ()) } }
};
}
