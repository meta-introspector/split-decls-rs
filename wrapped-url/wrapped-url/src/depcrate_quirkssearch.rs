// Generated macro for search (function)
macro_rules! Depcrate_quirkssearch {
() => {
// Module: crate::quirks
// Provides: {"search"}
// Dependencies: {}
# [doc = " Getter for <https://url.spec.whatwg.org/#dom-url-search>"] pub fn search (url : & Url) -> & str { trim (& url [Position :: AfterPath .. Position :: AfterQuery]) }
};
}
