// Generated macro for ident_ty (function)
macro_rules! Depcrate_utilident_ty {
() => {
// Module: crate::util
// Provides: {"ident_ty"}
// Dependencies: {}
# [doc = " Create a path type with a single segment from a given Identifier"] pub fn ident_ty (ident : Ident) -> syn :: Type { parse_quote ! (# ident) }
};
}
