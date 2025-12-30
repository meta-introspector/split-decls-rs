// Generated macro for serialize_hex_upper_or_bin (function)
macro_rules! Depcrate_commonserialize_hex_upper_or_bin {
() => {
// Module: crate::common
// Provides: {"serialize_hex_upper_or_bin"}
// Dependencies: {}
# [doc = " Serialize the given type as upper case hex when using human-readable"] # [doc = " formats or binary if the format is binary."] pub (crate) fn serialize_hex_upper_or_bin < S , T > (value : & T , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , T : AsRef < [u8] > , { if serializer . is_human_readable () { serialize_hex :: < _ , _ , true > (value , serializer) } else { serializer . serialize_bytes (value . as_ref ()) } }
};
}
