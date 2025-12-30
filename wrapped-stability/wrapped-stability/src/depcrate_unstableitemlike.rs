// Generated macro for ItemLike (trait)
macro_rules! Depcrate_unstableItemLike {
() => {
// Module: crate::unstable
// Provides: {"ItemLike"}
// Dependencies: {}
pub (crate) trait ItemLike { fn attrs (& self) -> & [syn :: Attribute] ; fn push_attr (& mut self , attr : syn :: Attribute) ; fn visibility (& self) -> & Visibility ; fn set_visibility (& mut self , visibility : Visibility) ; fn is_public (& self) -> bool { matches ! (self . visibility () , Visibility :: Public (_)) } }
};
}
