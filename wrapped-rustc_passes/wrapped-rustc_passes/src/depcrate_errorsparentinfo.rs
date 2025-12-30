// Generated macro for ParentInfo (struct)
macro_rules! Depcrate_errorsParentInfo {
() => {
// Module: crate::errors
// Provides: {"ParentInfo"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [label (passes_parent_info)] pub (crate) struct ParentInfo < 'tcx > { pub num : usize , pub descr : & 'tcx str , pub parent_descr : & 'tcx str , # [primary_span] pub span : Span , }
};
}
