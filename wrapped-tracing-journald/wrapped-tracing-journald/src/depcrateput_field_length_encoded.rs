// Generated macro for put_field_length_encoded (function)
macro_rules! Depcrateput_field_length_encoded {
() => {
// Module: crate
// Provides: {"put_field_length_encoded"}
// Dependencies: {}
# [doc = " Append a sanitized and length-encoded field into `buf`."] # [doc = ""] # [doc = " Unlike `put_field_wellformed` this function handles arbitrary field names and values."] # [doc = ""] # [doc = " `name` denotes the field name. It gets sanitized before being appended to `buf`."] # [doc = ""] # [doc = " `write_value` is invoked with `buf` as argument to append the value data to `buf`.  It must"] # [doc = " not delete from `buf`, but may append arbitrary data.  This function then determines the length"] # [doc = " of the data written and adds it in the appropriate place in `buf`."] fn put_field_length_encoded (buf : & mut Vec < u8 > , name : & str , write_value : impl FnOnce (& mut Vec < u8 >)) { sanitize_name (name , buf) ; buf . push (b'\n') ; buf . extend_from_slice (& [0 ; 8]) ; let start = buf . len () ; write_value (buf) ; let end = buf . len () ; buf [start - 8 .. start] . copy_from_slice (& ((end - start) as u64) . to_le_bytes ()) ; buf . push (b'\n') ; }
};
}
