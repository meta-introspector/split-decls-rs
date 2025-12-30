// Generated macro for impl_sysvar_id (macro)
macro_rules! Depcrateimpl_sysvar_id {
() => {
// Module: crate
// Provides: {"impl_sysvar_id"}
// Dependencies: {}
# [doc = " Implements [`SysvarId`] for a module that already uses"] # [doc = " `declare_id``"] # [macro_export] macro_rules ! impl_sysvar_id (($ type : ty) => { impl $ crate :: SysvarId for $ type { fn id () -> $ crate :: Address { id () } fn check_id (address : &$ crate :: Address) -> bool { check_id (address) } } }) ;
};
}
