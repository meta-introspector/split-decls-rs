// Generated macro for get (function)
macro_rules! Depcrate_tlvget {
() => {
// Module: crate::tlv
// Provides: {"get"}
// Dependencies: {}
# [doc = " Returns the current thread-local value"] # [inline] pub (crate) fn get () -> Tlv { TLV . with (| tlv | Tlv (tlv . get ())) }
};
}
