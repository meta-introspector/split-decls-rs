// Generated macro for expect_failed (function)
macro_rules! Depcrateexpect_failed {
() => {
// Module: crate
// Provides: {"expect_failed"}
// Dependencies: {}
# [doc = " This is a separate function to reduce the code size of `expect_opt!`."] # [inline (never)] # [cold] # [track_caller] const fn expect_failed (message : & str) -> ! { panic ! ("{}" , message) }
};
}
