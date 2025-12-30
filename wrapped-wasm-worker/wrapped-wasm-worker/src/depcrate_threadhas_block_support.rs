// Generated macro for has_block_support (function)
macro_rules! Depcrate_threadhas_block_support {
() => {
// Module: crate::thread
// Provides: {"has_block_support"}
// Dependencies: {}
# [doc = " Implementation for [`crate::web::has_block_support()`]."] pub (crate) fn has_block_support () -> bool { thread_local ! { static HAS_BLOCK_SUPPORT : bool = Global :: with (| global | { match global { Global :: Window (_) | Global :: Worklet | Global :: Service (_) => false , Global :: Dedicated (_) => true , Global :: Shared (_) => { # [doc = " Cache if blocking on shared workers is supported."] # [doc = " REASON: A Wasm module can never be shared between"] # [doc = " multiple shared workers, so this can never be"] # [doc = " initialized from multiple threads at the same time."] # [allow (clippy :: disallowed_methods)] static HAS_SHARED_WORKER_BLOCK_SUPPORT : OnceLock < bool > = OnceLock :: new () ; * HAS_SHARED_WORKER_BLOCK_SUPPORT . get_or_init (r#impl :: test_block_support) } Global :: Worker (_) | Global :: Unknown => r#impl :: test_block_support () , } }) ; } HAS_BLOCK_SUPPORT . with (bool :: clone) }
};
}
