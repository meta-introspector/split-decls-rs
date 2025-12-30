// Generated macro for StorageLiveLocals (struct)
macro_rules! Depcrate_ssaStorageLiveLocals {
() => {
// Module: crate::ssa
// Provides: {"StorageLiveLocals"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct StorageLiveLocals { # [doc = " Set of \"StorageLive\" statements for each local."] storage_live : IndexVec < Local , Set1 < DefLocation > > , }
};
}
