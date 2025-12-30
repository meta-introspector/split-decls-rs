// Generated macro for SafetyContext (enum)
macro_rules! Depcrate_check_unsafetySafetyContext {
() => {
// Module: crate::check_unsafety
// Provides: {"SafetyContext"}
// Dependencies: {}
# [derive (Clone)] enum SafetyContext { Safe , BuiltinUnsafeBlock , UnsafeFn , UnsafeBlock { span : Span , hir_id : HirId , used : bool , nested_used_blocks : Vec < NestedUsedBlock > } , }
};
}
