// Generated macro for impl_20 (impl)
macro_rules! Depcrate_iterimpl_20 {
() => {
// Module: crate::iter
// Provides: {"impl_20"}
// Dependencies: {}
impl Iterator for UncheckedIter { type Item = JsValue ; fn next (& mut self) -> Option < Self :: Item > { let next = self . 0 . next () . unwrap_throw () ; if next . done () { None } else { Some (next . value ()) } } }
};
}
