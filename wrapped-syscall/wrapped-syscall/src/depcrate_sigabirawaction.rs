// Generated macro for RawAction (struct)
macro_rules! Depcrate_sigabiRawAction {
() => {
// Module: crate::sigabi
// Provides: {"RawAction"}
// Dependencies: {}
# [derive (Debug , Default)] # [repr (C , align (16))] pub struct RawAction { # [doc = " Only two MSBs are interesting for the kernel. If bit 63 is set, signal is ignored. If bit"] # [doc = " 62 is set and the signal is SIGTSTP/SIGTTIN/SIGTTOU, it's equivalent to the action of"] # [doc = " Stop."] pub first : AtomicU64 , # [doc = " Completely ignored by the kernel, but exists so userspace can (when 16-byte atomics exist)"] # [doc = " atomically set both the handler, sigaction flags, and sigaction mask."] pub user_data : AtomicU64 , }
};
}
