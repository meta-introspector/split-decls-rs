// Generated macro for impl_17 (impl)
macro_rules! Depcrate_attrimpl_17 {
() => {
// Module: crate::attr
// Provides: {"impl_17"}
// Dependencies: {}
impl From < & Meta > for MetaStyle { fn from (meta : & Meta) -> Self { match meta { Meta :: Path (..) => MetaStyle :: Ident , Meta :: List (..) => MetaStyle :: List , Meta :: NameValue (..) => MetaStyle :: NameValue , } } }
};
}
