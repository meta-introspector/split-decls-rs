// Generated macro for impl_17 (impl)
macro_rules! Depcrate_mapimpl_17 {
() => {
// Module: crate::map
// Provides: {"impl_17"}
// Dependencies: {}
impl < K , V > Map < K , V > where K : Ord , { pub (crate) fn is_dotted (& self) -> bool { self . dotted } pub (crate) fn is_implicit (& self) -> bool { self . implicit } pub (crate) fn is_inline (& self) -> bool { self . inline } pub (crate) fn set_implicit (& mut self , yes : bool) { self . implicit = yes ; } pub (crate) fn set_dotted (& mut self , yes : bool) { self . dotted = yes ; } pub (crate) fn set_inline (& mut self , yes : bool) { self . inline = yes ; } }
};
}
