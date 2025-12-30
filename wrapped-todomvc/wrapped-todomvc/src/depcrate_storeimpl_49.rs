// Generated macro for impl_49 (impl)
macro_rules! Depcrate_storeimpl_49 {
() => {
// Module: crate::store
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a > ItemListTrait < & 'a Item > for ItemListSlice < 'a > { fn new () -> ItemListSlice < 'a > { ItemListSlice { list : Vec :: new () } } fn get (& self , i : usize) -> Option < & & 'a Item > { self . list . get (i) } fn length (& self) -> usize { self . list . len () } fn push (& mut self , item : & 'a Item) { self . list . push (item) } fn iter (& self) -> std :: slice :: Iter < '_ , & 'a Item > { self . list . iter () } }
};
}
