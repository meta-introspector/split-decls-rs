// Generated macro for rust_panic (function)
macro_rules! Depcrate_panickingrust_panic {
() => {
// Module: crate::panicking
// Provides: {"rust_panic"}
// Dependencies: {}
# [cfg_attr (not (test) , rustc_std_internal_symbol)] # [cfg (feature = "panic_immediate_abort")] fn rust_panic (_ : & mut dyn PanicPayload) -> ! { unsafe { crate :: intrinsics :: abort () ; } }
};
}
