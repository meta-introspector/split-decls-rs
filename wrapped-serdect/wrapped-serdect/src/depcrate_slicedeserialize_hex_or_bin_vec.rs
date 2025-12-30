// Generated macro for deserialize_hex_or_bin_vec (function)
macro_rules! Depcrate_slicedeserialize_hex_or_bin_vec {
() => {
// Module: crate::slice
// Provides: {"deserialize_hex_or_bin_vec"}
// Dependencies: {}
# [doc = " Deserialize from hex when using human-readable formats or binary if the"] # [doc = " format is binary."] # [cfg (feature = "alloc")] pub fn deserialize_hex_or_bin_vec < 'de , D > (deserializer : D) -> Result < Vec < u8 > , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { deserializer . deserialize_str (StrIntoVecVisitor) } else { deserializer . deserialize_byte_buf (VecVisitor) } }
};
}
