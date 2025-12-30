// Generated macro for test_block_support (function)
macro_rules! Depcrate_thread_atomicstest_block_support {
() => {
// Module: crate::thread::atomics
// Provides: {"test_block_support"}
// Dependencies: {}
# [doc = " Tests if blocking is supported."] pub (super) fn test_block_support () -> bool { let value = Pin :: new (& 0) ; let index = i32_to_buffer_index (ptr :: from_ref (& value)) ; MEMORY_ARRAY . with (| array | Atomics :: wait_with_timeout (array , index , 0 , 0.)) . is_ok () }
};
}
