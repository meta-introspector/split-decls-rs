// Generated macro for impl_14 (impl)
macro_rules! Depcrate_unstableimpl_14 {
() => {
// Module: crate::unstable
// Provides: {"impl_14"}
// Dependencies: {}
impl ItemLike for syn :: ItemStruct { fn attrs (& self) -> & [syn :: Attribute] { & self . attrs } fn push_attr (& mut self , attr : syn :: Attribute) { self . attrs . push (attr) ; } fn visibility (& self) -> & Visibility { & self . vis } fn set_visibility (& mut self , visibility : Visibility) { self . fields . iter_mut () . filter (| field | matches ! (& field . vis , Visibility :: Public (_))) . for_each (| field | field . vis = visibility . clone ()) ; self . vis = visibility ; } }
};
}
