// Generated macro for impl_39 (impl)
macro_rules! Depcrate_attr_custom_metaimpl_39 {
() => {
// Module: crate::attr::custom_meta
// Provides: {"impl_39"}
// Dependencies: {}
impl CustomMeta { pub fn path (& self) -> & Path { match self { CustomMeta :: Not (_not , path) => path , CustomMeta :: Path (path) => path , CustomMeta :: List (meta) => & meta . path , CustomMeta :: NameValue (meta) => & meta . path , } } }
};
}
