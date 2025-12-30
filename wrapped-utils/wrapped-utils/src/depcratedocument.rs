// Generated macro for document (function)
macro_rules! Depcratedocument {
() => {
// Module: crate
// Provides: {"document"}
// Dependencies: {}
# [doc = " Convenience function to access the web_sys DOM document."] pub fn document () -> web_sys :: Document { window () . document () . expect_throw ("Can't find document") }
};
}
