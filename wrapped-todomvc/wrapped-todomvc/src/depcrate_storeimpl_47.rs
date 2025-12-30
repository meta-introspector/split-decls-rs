// Generated macro for impl_47 (impl)
macro_rules! Depcrate_storeimpl_47 {
() => {
// Module: crate::store
// Provides: {"impl_47"}
// Dependencies: {}
impl FromIterator < Item > for ItemList { fn from_iter < I : IntoIterator < Item = Item > > (iter : I) -> Self { let mut c = ItemList :: new () ; for i in iter { c . push (i) ; } c } }
};
}
