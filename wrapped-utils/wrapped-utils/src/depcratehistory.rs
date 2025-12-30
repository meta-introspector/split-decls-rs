// Generated macro for history (function)
macro_rules! Depcratehistory {
() => {
// Module: crate
// Provides: {"history"}
// Dependencies: {}
# [doc = " Convenience function to access the web_sys history."] pub fn history () -> web_sys :: History { window () . history () . expect_throw ("Can't find history") }
};
}
