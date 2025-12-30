// Generated macro for impl_55 (impl)
macro_rules! Depcrate_bagimpl_55 {
() => {
// Module: crate::bag
// Provides: {"impl_55"}
// Dependencies: {}
impl < T , const ARRAY_LEN : usize > FromIterator < T > for Bag < T , ARRAY_LEN > { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { let into_iter = iter . into_iter () ; let bag = Self :: new () ; into_iter . for_each (| v | { bag . push (v) ; }) ; bag } }
};
}
