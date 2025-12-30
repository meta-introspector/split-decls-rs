// Generated macro for impl_47 (impl)
macro_rules! Depcrate_internals_attrimpl_47 {
() => {
// Module: crate::internals::attr
// Provides: {"impl_47"}
// Dependencies: {}
impl Identifier { # [cfg (feature = "deserialize_in_place")] pub fn is_some (self) -> bool { match self { Identifier :: No => false , Identifier :: Field | Identifier :: Variant => true , } } }
};
}
