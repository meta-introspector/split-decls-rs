// Generated macro for is_empty_infer (function)
macro_rules! Depcrate_itemsis_empty_infer {
() => {
// Module: crate::items
// Provides: {"is_empty_infer"}
// Dependencies: {}
fn is_empty_infer (ty : & ast :: Ty , pat_span : Span) -> bool { match ty . kind { ast :: TyKind :: Infer => ty . span . hi () == pat_span . hi () , _ => false , } }
};
}
