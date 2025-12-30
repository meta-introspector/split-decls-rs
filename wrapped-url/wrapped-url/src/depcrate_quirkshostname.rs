// Generated macro for hostname (function)
macro_rules! Depcrate_quirkshostname {
() => {
// Module: crate::quirks
// Provides: {"hostname"}
// Dependencies: {}
# [doc = " Getter for <https://url.spec.whatwg.org/#dom-url-hostname>"] # [inline] pub fn hostname (url : & Url) -> & str { url . host_str () . unwrap_or ("") }
};
}
