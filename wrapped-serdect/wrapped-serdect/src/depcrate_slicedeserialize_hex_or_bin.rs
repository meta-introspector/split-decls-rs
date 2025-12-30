// Generated macro for deserialize_hex_or_bin (function)
macro_rules! Depcrate_slicedeserialize_hex_or_bin {
() => {
// Module: crate::slice
// Provides: {"deserialize_hex_or_bin"}
// Dependencies: {}
# [doc = " Deserialize from hex when using human-readable formats or binary if the"] # [doc = " format is binary. Fails if the `buffer` is smaller then the resulting"] # [doc = " slice."] pub fn deserialize_hex_or_bin < 'de , D > (buffer : & mut [u8] , deserializer : D) -> Result < & [u8] , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { deserializer . deserialize_str (StrIntoBufVisitor :: < UpperBound > (buffer , PhantomData)) } else { deserializer . deserialize_byte_buf (SliceVisitor :: < UpperBound > (buffer , PhantomData)) } }
};
}
