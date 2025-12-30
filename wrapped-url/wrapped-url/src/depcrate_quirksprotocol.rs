// Generated macro for protocol (function)
macro_rules! Depcrate_quirksprotocol {
() => {
// Module: crate::quirks
// Provides: {"protocol"}
// Dependencies: {}
# [doc = " Getter for <https://url.spec.whatwg.org/#dom-url-protocol>"] # [inline] pub fn protocol (url : & Url) -> & str { & url . as_str () [.. url . scheme () . len () + ":" . len ()] }
};
}
