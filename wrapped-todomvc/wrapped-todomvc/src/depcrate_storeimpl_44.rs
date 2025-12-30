// Generated macro for impl_44 (impl)
macro_rules! Depcrate_storeimpl_44 {
() => {
// Module: crate::store
// Provides: {"impl_44"}
// Dependencies: {}
impl ItemList { fn retain < F > (& mut self , f : F) where F : FnMut (& Item) -> bool , { self . list . retain (f) ; } fn iter_mut (& mut self) -> std :: slice :: IterMut < '_ , Item > { self . list . iter_mut () } }
};
}
