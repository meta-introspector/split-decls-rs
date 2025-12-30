// Generated macro for impl_152 (impl)
macro_rules! Depcrate_itemimpl_152 {
() => {
// Module: crate::item
// Provides: {"impl_152"}
// Dependencies: {}
impl Clone for Item { # [inline (never)] fn clone (& self) -> Self { match self { Self :: None => Self :: None , Self :: Value (v) => Self :: Value (v . clone ()) , Self :: Table (v) => Self :: Table (v . clone ()) , Self :: ArrayOfTables (v) => Self :: ArrayOfTables (v . clone ()) , } } }
};
}
