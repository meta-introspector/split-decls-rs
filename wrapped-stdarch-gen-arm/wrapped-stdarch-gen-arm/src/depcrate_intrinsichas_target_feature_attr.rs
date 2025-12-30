// Generated macro for has_target_feature_attr (function)
macro_rules! Depcrate_intrinsichas_target_feature_attr {
() => {
// Module: crate::intrinsic
// Provides: {"has_target_feature_attr"}
// Dependencies: {}
fn has_target_feature_attr (attrs : & [Expression]) -> bool { attrs . iter () . any (| attr | { if let Expression :: FnCall (fn_call) = attr { fn_call . is_target_feature_call () } else { false } }) }
};
}
