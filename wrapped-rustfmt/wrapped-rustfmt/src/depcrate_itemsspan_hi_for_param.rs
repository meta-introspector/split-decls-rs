// Generated macro for span_hi_for_param (function)
macro_rules! Depcrate_itemsspan_hi_for_param {
() => {
// Module: crate::items
// Provides: {"span_hi_for_param"}
// Dependencies: {}
pub (crate) fn span_hi_for_param (context : & RewriteContext < '_ > , param : & ast :: Param) -> BytePos { match param . ty . kind { ast :: TyKind :: Infer if context . snippet (param . ty . span) == "_" => param . ty . span . hi () , ast :: TyKind :: Infer if is_named_param (param) => param . pat . span . hi () , _ => param . ty . span . hi () , } }
};
}
