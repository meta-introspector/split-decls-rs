// Generated macro for sleep_ms (function)
macro_rules! Depcrate_threadsleep_ms {
() => {
// Module: crate::thread
// Provides: {"sleep_ms"}
// Dependencies: {}
# [doc = " See [`std::thread::sleep_ms()`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This call will panic if the calling thread doesn't support blocking, see"] # [doc = " [`web::has_block_support()`](crate::web::has_block_support)."] # [deprecated (note = "replaced by `web_thread::sleep`")] pub fn sleep_ms (ms : u32) { sleep (Duration :: from_millis (ms . into ())) ; }
};
}
