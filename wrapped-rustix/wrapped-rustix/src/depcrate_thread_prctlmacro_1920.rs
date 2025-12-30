// Generated macro for macro_1920 (macro)
macro_rules! Depcrate_thread_prctlmacro_1920 {
() => {
// Module: crate::thread::prctl
// Provides: {"macro_1920"}
// Dependencies: {}
bitflags ! { # [doc = " Zero means addresses that are passed for the purpose of being"] # [doc = " dereferenced by the kernel must be untagged."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct TaggedAddressMode : u32 { # [doc = " Addresses that are passed for the purpose of being dereferenced by"] # [doc = " the kernel may be tagged."] const ENABLED = 1_u32 << 0 ; # [doc = " Synchronous tag check fault mode."] const TCF_SYNC = 1_u32 << 1 ; # [doc = " Asynchronous tag check fault mode."] const TCF_ASYNC = 1_u32 << 2 ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
