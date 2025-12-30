// Generated macro for init_with_runtime (function)
macro_rules! Depcrate_androidinit_with_runtime {
() => {
// Module: crate::android
// Provides: {"init_with_runtime"}
// Dependencies: {}
# [doc = " Initialize with a runtime that can dynamically serve references to"] # [doc = " the JVM, context, and class loader."] # [doc = ""] # [doc = " This is the most flexible option, and is useful for advanced use cases."] # [doc = ""] # [doc = " This function will never panic."] pub fn init_with_runtime (runtime : & 'static dyn Runtime) { GLOBAL . get_or_init (| | Global :: External (runtime)) ; }
};
}
