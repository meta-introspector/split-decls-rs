// Generated macro for body (function)
macro_rules! Depcratebody {
() => {
// Module: crate
// Provides: {"body"}
// Dependencies: {}
# [doc = " Convenience function to access `document.body`."] pub fn body () -> web_sys :: HtmlElement { document () . body () . expect_throw ("Can't find document body") }
};
}
