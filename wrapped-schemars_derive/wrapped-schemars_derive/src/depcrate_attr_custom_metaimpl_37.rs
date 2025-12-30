// Generated macro for impl_37 (impl)
macro_rules! Depcrate_attr_custom_metaimpl_37 {
() => {
// Module: crate::attr::custom_meta
// Provides: {"impl_37"}
// Dependencies: {}
impl From < Meta > for CustomMeta { fn from (value : Meta) -> Self { match value { Meta :: Path (meta) => Self :: Path (meta) , Meta :: List (meta) => Self :: List (meta) , Meta :: NameValue (meta) => Self :: NameValue (meta) , } } }
};
}
