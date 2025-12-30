// Generated macro for port (function)
macro_rules! Depcrate_quirksport {
() => {
// Module: crate::quirks
// Provides: {"port"}
// Dependencies: {}
# [doc = " Getter for <https://url.spec.whatwg.org/#dom-url-port>"] # [inline] pub fn port (url : & Url) -> & str { & url [Position :: BeforePort .. Position :: AfterPort] }
};
}
