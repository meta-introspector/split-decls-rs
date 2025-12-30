// Generated macro for set (function)
macro_rules! Depcrate_tlvset {
() => {
// Module: crate::tlv
// Provides: {"set"}
// Dependencies: {}
# [doc = " Sets the current thread-local value"] # [inline] pub (crate) fn set (value : Tlv) { TLV . with (| tlv | tlv . set (value . 0)) ; }
};
}
