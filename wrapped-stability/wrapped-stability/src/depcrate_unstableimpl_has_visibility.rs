// Generated macro for impl_has_visibility (macro)
macro_rules! Depcrate_unstableimpl_has_visibility {
() => {
// Module: crate::unstable
// Provides: {"impl_has_visibility"}
// Dependencies: {}
macro_rules ! impl_has_visibility { ($ ($ ty : ty) ,+ $ (,) ?) => { $ (impl ItemLike for $ ty { fn attrs (& self) -> & [syn :: Attribute] { & self . attrs } fn push_attr (& mut self , attr : syn :: Attribute) { self . attrs . push (attr) ; } fn visibility (& self) -> & Visibility { & self . vis } fn set_visibility (& mut self , visibility : Visibility) { self . vis = visibility ; } }) * } ; }
};
}
