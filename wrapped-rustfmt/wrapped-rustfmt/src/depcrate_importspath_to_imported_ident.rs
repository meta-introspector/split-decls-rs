// Generated macro for path_to_imported_ident (function)
macro_rules! Depcrate_importspath_to_imported_ident {
() => {
// Module: crate::imports
// Provides: {"path_to_imported_ident"}
// Dependencies: {}
# [doc = " Returns a name imported by a `use` declaration."] # [doc = " E.g., returns `Ordering` for `std::cmp::Ordering` and `self` for `std::cmp::self`."] pub (crate) fn path_to_imported_ident (path : & ast :: Path) -> symbol :: Ident { path . segments . last () . unwrap () . ident }
};
}
