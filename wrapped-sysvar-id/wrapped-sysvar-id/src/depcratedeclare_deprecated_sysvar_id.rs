// Generated macro for declare_deprecated_sysvar_id (macro)
macro_rules! Depcratedeclare_deprecated_sysvar_id {
() => {
// Module: crate
// Provides: {"declare_deprecated_sysvar_id"}
// Dependencies: {}
# [doc = " Same as [`declare_sysvar_id`] except that it reports that this ID has been deprecated."] # [macro_export] macro_rules ! declare_deprecated_sysvar_id (($ name : expr , $ type : ty) => ($ crate :: declare_deprecated_id ! ($ name) ; $ crate :: impl_deprecated_sysvar_id ! ($ type) ;)) ;
};
}
