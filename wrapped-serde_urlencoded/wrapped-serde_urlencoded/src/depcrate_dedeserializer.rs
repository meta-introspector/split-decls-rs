// Generated macro for Deserializer (struct)
macro_rules! Depcrate_deDeserializer {
() => {
// Module: crate::de
// Provides: {"Deserializer"}
// Dependencies: {}
# [doc = " A deserializer for the `application/x-www-form-urlencoded` format."] # [doc = ""] # [doc = " * Supported top-level outputs are structs, maps and sequences of pairs,"] # [doc = "   with or without a given length."] # [doc = ""] # [doc = " * Main `deserialize` methods defers to `deserialize_map`."] # [doc = ""] # [doc = " * Everything else but `deserialize_seq` and `deserialize_seq_fixed_size`"] # [doc = "   defers to `deserialize`."] pub struct Deserializer < 'de > { inner : MapDeserializer < 'de , PartIterator < 'de > , Error > , }
};
}
