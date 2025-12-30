// Generated macro for SigProcControl (struct)
macro_rules! Depcrate_sigabiSigProcControl {
() => {
// Module: crate::sigabi
// Provides: {"SigProcControl"}
// Dependencies: {}
# [doc = " Signal runtime struct for the entire process"] # [derive (Debug)] # [repr (C , align (4096))] pub struct SigProcControl { pub pending : AtomicU64 , pub actions : [RawAction ; 64] , pub sender_infos : [AtomicU64 ; 32] , }
};
}
