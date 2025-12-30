// Generated macro for Sigcontrol (struct)
macro_rules! Depcrate_sigabiSigcontrol {
() => {
// Module: crate::sigabi
// Provides: {"Sigcontrol"}
// Dependencies: {}
# [doc = " Signal runtime struct for a thread"] # [derive (Debug , Default)] # [repr (C)] pub struct Sigcontrol { pub word : [AtomicU64 ; 2] , pub sender_infos : [AtomicU64 ; 32] , pub control_flags : SigatomicUsize , pub saved_ip : NonatomicUsize , pub saved_archdep_reg : NonatomicUsize , }
};
}
