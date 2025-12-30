// Generated macro for leading_colon_path_ty (function)
macro_rules! Depcrate_utilleading_colon_path_ty {
() => {
// Module: crate::util
// Provides: {"leading_colon_path_ty"}
// Dependencies: {}
# [doc = " Create a global path type from the given segments. For example an iterator"] # [doc = " yielding the idents `[foo, bar, baz]` will result in the path type"] # [doc = " `::foo::bar::baz`."] pub fn leading_colon_path_ty < I > (segments : I) -> syn :: Type where I : IntoIterator < Item = Ident > , { let segments = segments . into_iter () ; parse_quote ! (::# (# segments) ::*) }
};
}
