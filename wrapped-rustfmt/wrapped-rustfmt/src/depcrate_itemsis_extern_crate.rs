// Generated macro for is_extern_crate (function)
macro_rules! Depcrate_itemsis_extern_crate {
() => {
// Module: crate::items
// Provides: {"is_extern_crate"}
// Dependencies: {}
pub (crate) fn is_extern_crate (item : & ast :: Item) -> bool { matches ! (item . kind , ast :: ItemKind :: ExternCrate (..)) }
};
}
