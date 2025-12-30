// Generated macro for deserialize_hex_or_bin (function)
macro_rules! Depcrate_arraydeserialize_hex_or_bin {
() => {
// Module: crate::array
// Provides: {"deserialize_hex_or_bin"}
// Dependencies: {}
# [doc = " Deserialize from hex when using human-readable formats or binary if the"] # [doc = " format is binary. Fails if the `buffer` isn't the exact same size as the"] # [doc = " resulting array."] pub fn deserialize_hex_or_bin < 'de , D > (buffer : & mut [u8] , deserializer : D) -> Result < & [u8] , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { deserializer . deserialize_str (StrIntoBufVisitor :: < ExactLength > (buffer , PhantomData)) } else { deserializer . deserialize_byte_buf (SliceVisitor :: < ExactLength > (buffer , PhantomData)) } }
};
}
