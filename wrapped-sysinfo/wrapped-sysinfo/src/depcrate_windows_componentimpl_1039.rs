// Generated macro for impl_1039 (impl)
macro_rules! Depcrate_windows_componentimpl_1039 {
() => {
// Module: crate::windows::component
// Provides: {"impl_1039"}
// Dependencies: {}
impl ComponentsInner { pub (crate) fn new () -> Self { Self { components : Vec :: new () , } } pub (crate) fn from_vec (components : Vec < Component >) -> Self { Self { components } } pub (crate) fn into_vec (self) -> Vec < Component > { self . components } pub (crate) fn list (& self) -> & [Component] { & self . components } pub (crate) fn list_mut (& mut self) -> & mut [Component] { & mut self . components } pub (crate) fn refresh (& mut self) { if self . components . is_empty () { self . components = match ComponentInner :: new () { Some (c) => vec ! [Component { inner : c }] , None => Vec :: new () , } ; } else { for c in self . components . iter_mut () { c . refresh () ; c . inner . updated = true ; } } } }
};
}
