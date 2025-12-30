// Generated macro for ItemListTrait (trait)
macro_rules! Depcrate_storeItemListTrait {
() => {
// Module: crate::store
// Provides: {"ItemListTrait"}
// Dependencies: {}
pub trait ItemListTrait < T > { fn new () -> Self ; fn get (& self , i : usize) -> Option < & T > ; fn length (& self) -> usize ; fn push (& mut self , item : T) ; fn iter (& self) -> std :: slice :: Iter < '_ , T > ; }
};
}
