// Generated macro for SyncState (struct)
macro_rules! Depcrate_ansiSyncState {
() => {
// Module: crate::ansi
// Provides: {"SyncState"}
// Dependencies: {}
# [derive (Debug)] struct SyncState < T : Timeout > { # [doc = " Handler for synchronized updates."] timeout : T , # [doc = " Bytes read during the synchronized update."] buffer : Vec < u8 > , }
};
}
