// Generated macro for host (function)
macro_rules! Depcrate_quirkshost {
() => {
// Module: crate::quirks
// Provides: {"host"}
// Dependencies: {}
# [doc = " Getter for <https://url.spec.whatwg.org/#dom-url-host>"] # [inline] pub fn host (url : & Url) -> & str { & url [Position :: BeforeHost .. Position :: AfterPort] }
};
}
