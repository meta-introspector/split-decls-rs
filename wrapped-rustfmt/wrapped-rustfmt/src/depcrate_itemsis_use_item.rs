// Generated macro for is_use_item (function)
macro_rules! Depcrate_itemsis_use_item {
() => {
// Module: crate::items
// Provides: {"is_use_item"}
// Dependencies: {}
pub (crate) fn is_use_item (item : & ast :: Item) -> bool { matches ! (item . kind , ast :: ItemKind :: Use (_)) }
};
}
