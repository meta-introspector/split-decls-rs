// Generated macro for document_element (function)
macro_rules! Depcratedocument_element {
() => {
// Module: crate
// Provides: {"document_element"}
// Dependencies: {}
# [doc = " Convenience function to access `document.documentElement`."] pub fn document_element () -> web_sys :: Element { document () . document_element () . expect_throw ("Can't find document element") }
};
}
