// Generated macro for impl_25 (impl)
macro_rules! Depcrate_anymapimpl_25 {
() => {
// Module: crate::anymap
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'map , A : ? Sized + Downcast , V : IntoBox < A > > OccupiedEntry < 'map , A , V > { # [doc = " Converts the `OccupiedEntry` into a mutable reference to the value in the entry"] # [doc = " with a lifetime bound to the collection itself"] # [inline] # [must_use] pub fn into_mut (self) -> & 'map mut V { unsafe { self . inner . into_mut () . downcast_unchecked_mut () } } }
};
}
