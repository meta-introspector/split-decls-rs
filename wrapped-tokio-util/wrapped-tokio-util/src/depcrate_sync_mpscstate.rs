// Generated macro for State (enum)
macro_rules! Depcrate_sync_mpscState {
() => {
// Module: crate::sync::mpsc
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug)] enum State < T > { Idle (Sender < T >) , Acquiring , ReadyToSend (OwnedPermit < T >) , Closed , }
};
}
