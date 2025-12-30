// Generated macro for put_value (function)
macro_rules! Depcrateput_value {
() => {
// Module: crate
// Provides: {"put_value"}
// Dependencies: {}
# [doc = " Write the value portion of a key-value pair, in newline separated format."] # [doc = ""] # [doc = " `value` must not contain an internal newline."] # [doc = ""] # [doc = " For a \"newline-safe\" variant, see `put_field_length_encoded`."] fn put_value (buf : & mut Vec < u8 > , value : & [u8]) { buf . extend_from_slice (& (value . len () as u64) . to_le_bytes ()) ; buf . extend_from_slice (value) ; buf . push (b'\n') ; }
};
}
