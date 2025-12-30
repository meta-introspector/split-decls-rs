// Generated macro for serialize_hex_upper_or_bin (function)
macro_rules! Depcrate_arrayserialize_hex_upper_or_bin {
() => {
// Module: crate::array
// Provides: {"serialize_hex_upper_or_bin"}
// Dependencies: {}
# [doc = " Serialize the given type as upper case hex when using human-readable"] # [doc = " formats or binary if the format is binary."] pub fn serialize_hex_upper_or_bin < S , T > (value : & T , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , T : AsRef < [u8] > , { common :: serialize_hex_upper_or_bin (value , serializer) }
};
}
