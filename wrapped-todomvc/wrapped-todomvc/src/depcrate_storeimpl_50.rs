// Generated macro for impl_50 (impl)
macro_rules! Depcrate_storeimpl_50 {
() => {
// Module: crate::store
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a > FromIterator < & 'a Item > for ItemListSlice < 'a > { fn from_iter < I : IntoIterator < Item = & 'a Item > > (iter : I) -> Self { let mut c = ItemListSlice :: new () ; for i in iter { c . push (i) ; } c } }
};
}
