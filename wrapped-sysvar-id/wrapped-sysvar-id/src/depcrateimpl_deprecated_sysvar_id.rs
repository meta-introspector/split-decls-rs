// Generated macro for impl_deprecated_sysvar_id (macro)
macro_rules! Depcrateimpl_deprecated_sysvar_id {
() => {
// Module: crate
// Provides: {"impl_deprecated_sysvar_id"}
// Dependencies: {}
# [doc = " Implements [`SysvarId`] for a module that already uses"] # [doc = " `declare_deprecated_id``"] # [macro_export] macro_rules ! impl_deprecated_sysvar_id (($ type : ty) => { impl $ crate :: SysvarId for $ type { fn id () -> $ crate :: Address { # [allow (deprecated)] id () } fn check_id (address : &$ crate :: Address) -> bool { # [allow (deprecated)] check_id (address) } } }) ;
};
}
