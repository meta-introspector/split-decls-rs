// Generated macro for EMPTY_PANIC (static)
macro_rules! Depcrate_panickingEMPTY_PANIC {
() => {
// Module: crate::panicking
// Provides: {"EMPTY_PANIC"}
// Dependencies: {}
# [unstable (feature = "libstd_sys_internals" , reason = "used by the panic! macro" , issue = "none")] # [doc (hidden)] # [allow (dead_code)] # [used (compiler)] pub static EMPTY_PANIC : fn (& 'static str) -> ! = begin_panic :: < & 'static str > as fn (& 'static str) -> ! ;
};
}
