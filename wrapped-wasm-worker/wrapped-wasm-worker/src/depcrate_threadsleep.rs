// Generated macro for sleep (function)
macro_rules! Depcrate_threadsleep {
() => {
// Module: crate::thread
// Provides: {"sleep"}
// Dependencies: {}
# [doc = " See [`std::thread::sleep()`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This call will panic if the calling thread doesn't support blocking, see"] # [doc = " [`web::has_block_support()`](crate::web::has_block_support)."] pub fn sleep (dur : Duration) { if has_block_support () { r#impl :: sleep (dur) ; } else { panic ! ("current thread type cannot be blocked") } }
};
}
