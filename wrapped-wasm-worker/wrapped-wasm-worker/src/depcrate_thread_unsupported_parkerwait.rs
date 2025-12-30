// Generated macro for wait (function)
macro_rules! Depcrate_thread_unsupported_parkerwait {
() => {
// Module: crate::thread::unsupported::parker
// Provides: {"wait"}
// Dependencies: {}
# [doc = " Wait a specified duration."] fn wait (timeout : Option < Duration >) { let timeout = timeout . map_or (f64 :: INFINITY , super :: duration_to_f64_millis) ; let result = ZERO_ARRAY . with (| array | Atomics :: wait_with_timeout (array , 0 , 0 , timeout)) . expect ("`Atomic.wait` is not expected to fail") ; debug_assert_eq ! (result , "timed-out" , "unexpected return value from `Atomics.wait") ; }
};
}
