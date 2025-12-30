// Generated macro for put_field_wellformed (function)
macro_rules! Depcrateput_field_wellformed {
() => {
// Module: crate
// Provides: {"put_field_wellformed"}
// Dependencies: {}
# [doc = " Append arbitrary data with a well-formed name and value."] # [doc = ""] # [doc = " `value` must not contain an internal newline, because this function writes"] # [doc = " `value` in the new-line separated format."] # [doc = ""] # [doc = " For a \"newline-safe\" variant, see `put_field_length_encoded`."] fn put_field_wellformed (buf : & mut Vec < u8 > , name : & str , value : & [u8]) { buf . extend_from_slice (name . as_bytes ()) ; buf . push (b'\n') ; put_value (buf , value) ; }
};
}
