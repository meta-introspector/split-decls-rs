// Generated macro for impl_964 (impl)
macro_rules! Depcrate_modulesimpl_964 {
() => {
// Module: crate::modules
// Provides: {"impl_964"}
// Dependencies: {}
impl < 'a > Module < 'a > { pub (crate) fn new (mod_span : Span , ast_mod_kind : Option < Cow < 'a , ast :: ModKind > > , mod_items : Cow < 'a , ThinVec < Box < ast :: Item > > > , mod_attrs : Cow < 'a , ast :: AttrVec > ,) -> Self { let inner_attr = mod_attrs . iter () . filter (| attr | attr . style == ast :: AttrStyle :: Inner) . cloned () . collect () ; Module { items : mod_items , inner_attr , span : mod_span , ast_mod_kind , } } pub (crate) fn attrs (& self) -> & [ast :: Attribute] { & self . inner_attr } }
};
}
