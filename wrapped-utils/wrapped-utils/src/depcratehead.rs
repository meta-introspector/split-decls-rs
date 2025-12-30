// Generated macro for head (function)
macro_rules! Depcratehead {
() => {
// Module: crate
// Provides: {"head"}
// Dependencies: {}
# [doc = " Convenience function to access the head element."] pub fn head () -> web_sys :: HtmlHeadElement { document () . head () . expect_throw ("Can't find the head element") }
};
}
