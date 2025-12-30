// Generated macro for get_attrs_from_stmt (function)
macro_rules! Depcrate_attrget_attrs_from_stmt {
() => {
// Module: crate::attr
// Provides: {"get_attrs_from_stmt"}
// Dependencies: {}
# [doc = " Returns attributes on the given statement."] pub (crate) fn get_attrs_from_stmt (stmt : & ast :: Stmt) -> & [ast :: Attribute] { stmt . attrs () }
};
}
