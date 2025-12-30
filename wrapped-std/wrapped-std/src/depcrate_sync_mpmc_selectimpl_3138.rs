// Generated macro for impl_3138 (impl)
macro_rules! Depcrate_sync_mpmc_selectimpl_3138 {
() => {
// Module: crate::sync::mpmc::select
// Provides: {"impl_3138"}
// Dependencies: {}
impl From < usize > for Selected { # [inline] fn from (val : usize) -> Selected { match val { 0 => Selected :: Waiting , 1 => Selected :: Aborted , 2 => Selected :: Disconnected , oper => Selected :: Operation (Operation (oper)) , } } }
};
}
