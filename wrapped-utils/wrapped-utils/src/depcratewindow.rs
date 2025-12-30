// Generated macro for window (function)
macro_rules! Depcratewindow {
() => {
// Module: crate
// Provides: {"window"}
// Dependencies: {}
# [doc = " Convenience function to avoid repeating expect logic."] pub fn window () -> web_sys :: Window { web_sys :: window () . expect_throw ("Can't find the global Window") }
};
}
