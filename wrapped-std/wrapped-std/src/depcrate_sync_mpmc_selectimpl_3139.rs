// Generated macro for impl_3139 (impl)
macro_rules! Depcrate_sync_mpmc_selectimpl_3139 {
() => {
// Module: crate::sync::mpmc::select
// Provides: {"impl_3139"}
// Dependencies: {}
impl Into < usize > for Selected { # [inline] fn into (self) -> usize { match self { Selected :: Waiting => 0 , Selected :: Aborted => 1 , Selected :: Disconnected => 2 , Selected :: Operation (Operation (val)) => val , } } }
};
}
