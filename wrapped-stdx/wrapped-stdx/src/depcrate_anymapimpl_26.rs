// Generated macro for impl_26 (impl)
macro_rules! Depcrate_anymapimpl_26 {
() => {
// Module: crate::anymap
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'map , A : ? Sized + Downcast , V : IntoBox < A > > VacantEntry < 'map , A , V > { # [doc = " Sets the value of the entry with the `VacantEntry`'s key,"] # [doc = " and returns a mutable reference to it"] # [inline] pub fn insert (self , value : V) -> & 'map mut V { unsafe { self . inner . insert (value . into_box ()) . downcast_unchecked_mut () } } }
};
}
