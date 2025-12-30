// Generated macro for sleep (function)
macro_rules! Depcrate_futuresleep {
() => {
// Module: crate::future
// Provides: {"sleep"}
// Dependencies: {}
# [doc = " Waits until the specified duration has elapsed."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if the specified [`Duration`] cannot be casted into a u32 in"] # [doc = " milliseconds."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " use std::time::Duration;"] # [doc = " use gloo_timers::future::sleep;"] # [doc = ""] # [doc = " sleep(Duration::from_secs(1)).await;"] # [doc = " ```"] pub fn sleep (dur : Duration) -> TimeoutFuture { let millis = u32 :: try_from (dur . as_millis ()) . expect_throw ("failed to cast the duration into a u32 with Duration::as_millis.") ; TimeoutFuture :: new (millis) }
};
}
