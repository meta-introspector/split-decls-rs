// Generated macro for impl_52 (impl)
macro_rules! Depcrate_ansiimpl_52 {
() => {
// Module: crate::ansi
// Provides: {"impl_52"}
// Dependencies: {}
impl < T : Timeout > Default for SyncState < T > { fn default () -> Self { Self { buffer : Vec :: with_capacity (SYNC_BUFFER_SIZE) , timeout : Default :: default () } } }
};
}
