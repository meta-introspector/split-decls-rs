// Generated macro for uWrite (trait)
macro_rules! DepcrateuWrite {
() => {
// Module: crate
// Provides: {"uWrite"}
// Dependencies: {}
# [doc = " A collection of methods that are required / used to format a message into a stream."] # [allow (non_camel_case_types)] pub trait uWrite { # [doc = " The error associated to this writer"] type Error ; # [doc = " Writes a string slice into this writer, returning whether the write succeeded."] # [doc = ""] # [doc = " This method can only succeed if the entire string slice was successfully written, and this"] # [doc = " method will not return until all data has been written or an error occurs."] fn write_str (& mut self , s : & str) -> Result < () , Self :: Error > ; # [doc = " Writes a [`char`] into this writer, returning whether the write succeeded."] # [doc = ""] # [doc = " A single [`char`] may be encoded as more than one byte. This method can only succeed if the"] # [doc = " entire byte sequence was successfully written, and this method will not return until all"] # [doc = " data has been written or an error occurs."] fn write_char (& mut self , c : char) -> Result < () , Self :: Error > { let mut buf : [u8 ; 4] = unsafe { uninitialized () } ; self . write_str (c . encode_utf8 (& mut buf)) } }
};
}
