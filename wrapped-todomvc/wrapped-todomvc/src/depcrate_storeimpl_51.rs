// Generated macro for impl_51 (impl)
macro_rules! Depcrate_storeimpl_51 {
() => {
// Module: crate::store
// Provides: {"impl_51"}
// Dependencies: {}
impl From < ItemListSlice < '_ > > for ItemList { fn from (s : ItemListSlice < '_ >) -> Self { let mut i = ItemList :: new () ; let items = s . list . into_iter () ; for j in items { let item = Item { id : j . id . clone () , completed : j . completed , title : j . title . clone () , } ; i . push (item) ; } i } }
};
}
