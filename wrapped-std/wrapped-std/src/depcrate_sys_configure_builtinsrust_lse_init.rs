// Generated macro for RUST_LSE_INIT (static)
macro_rules! Depcrate_sys_configure_builtinsRUST_LSE_INIT {
() => {
// Module: crate::sys::configure_builtins
// Provides: {"RUST_LSE_INIT"}
// Dependencies: {}
# [doc = " Hook into .init_array to enable LSE atomic operations at startup, if"] # [doc = " supported."] # [cfg (all (target_arch = "aarch64" , target_os = "linux" , not (feature = "compiler-builtins-c")))] # [used] # [unsafe (link_section = ".init_array.90")] static RUST_LSE_INIT : extern "C" fn () = { extern "C" fn init_lse () { use crate :: arch ; unsafe extern "C" { fn __rust_enable_lse () ; } if arch :: is_aarch64_feature_detected ! ("lse") { unsafe { __rust_enable_lse () ; } } } init_lse } ;
};
}
