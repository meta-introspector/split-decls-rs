// Generated macro for password (function)
macro_rules! Depcrate_quirkspassword {
() => {
// Module: crate::quirks
// Provides: {"password"}
// Dependencies: {}
# [doc = " Getter for <https://url.spec.whatwg.org/#dom-url-password>"] # [inline] pub fn password (url : & Url) -> & str { url . password () . unwrap_or ("") }
};
}
