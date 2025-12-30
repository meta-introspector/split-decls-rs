// Generated macro for declare_sysvar_id (macro)
macro_rules! Depcratedeclare_sysvar_id {
() => {
// Module: crate
// Provides: {"declare_sysvar_id"}
// Dependencies: {}
# [doc = " Declares an ID that implements [`SysvarId`]."] # [macro_export] macro_rules ! declare_sysvar_id (($ name : expr , $ type : ty) => ($ crate :: declare_id ! ($ name) ; $ crate :: impl_sysvar_id ! ($ type) ;)) ;
};
}
