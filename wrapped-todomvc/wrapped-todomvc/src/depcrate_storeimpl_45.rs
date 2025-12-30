// Generated macro for impl_45 (impl)
macro_rules! Depcrate_storeimpl_45 {
() => {
// Module: crate::store
// Provides: {"impl_45"}
// Dependencies: {}
impl ItemListTrait < Item > for ItemList { fn new () -> ItemList { ItemList { list : Vec :: new () } } fn get (& self , i : usize) -> Option < & Item > { self . list . get (i) } fn length (& self) -> usize { self . list . len () } fn push (& mut self , item : Item) { self . list . push (item) } fn iter (& self) -> std :: slice :: Iter < '_ , Item > { self . list . iter () } }
};
}
