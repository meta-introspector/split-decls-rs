// Generated macro for sleep (function)
macro_rules! Depcrate_thread_unsupportedsleep {
() => {
// Module: crate::thread::unsupported
// Provides: {"sleep"}
// Dependencies: {}
# [doc = " Implementation of [`std::thread::sleep()`]."] pub (super) fn sleep (dur : Duration) { let timeout = duration_to_f64_millis (dur) ; let result = ZERO_ARRAY . with (| array | Atomics :: wait_with_timeout (array , 0 , 0 , timeout)) . expect ("`Atomics.wait` is not expected to fail") ; debug_assert_eq ! (result , "timed-out" , "unexpected return value from `Atomics.wait") ; }
};
}
