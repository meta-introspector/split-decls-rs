// Generated macro for impl_109 (impl)
macro_rules! Depcrate_arc_unionimpl_109 {
() => {
// Module: crate::arc_union
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'a , A , B > ArcUnionBorrow < 'a , A , B > { # [doc = " The reference count of this `Arc`."] # [doc = ""] # [doc = " The number does not include borrowed pointers,"] # [doc = " or temporary `Arc` pointers created with functions like"] # [doc = " [`ArcBorrow::with_arc`]."] # [doc = ""] # [doc = " The function is called `strong_count` to mirror `std::sync::Arc::strong_count`,"] # [doc = " however `triomphe::Arc` does not support weak references."] # [inline] pub fn strong_count (this : & Self) -> usize { match this { ArcUnionBorrow :: First (arc) => ArcBorrow :: strong_count (arc) , ArcUnionBorrow :: Second (arc) => ArcBorrow :: strong_count (arc) , } } }
};
}
