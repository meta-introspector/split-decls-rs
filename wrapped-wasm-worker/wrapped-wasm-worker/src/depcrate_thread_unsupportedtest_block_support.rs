// Generated macro for test_block_support (function)
macro_rules! Depcrate_thread_unsupportedtest_block_support {
() => {
// Module: crate::thread::unsupported
// Provides: {"test_block_support"}
// Dependencies: {}
# [doc = " Tests if blocking is supported."] pub (super) fn test_block_support () -> bool { ZERO_ARRAY . with (| array | Atomics :: wait_with_timeout (array , 0 , 0 , 0.) . is_ok ()) }
};
}
