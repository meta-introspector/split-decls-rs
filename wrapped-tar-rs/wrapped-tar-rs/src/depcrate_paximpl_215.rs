// Generated macro for impl_215 (impl)
macro_rules! Depcrate_paximpl_215 {
() => {
// Module: crate::pax
// Provides: {"impl_215"}
// Dependencies: {}
impl < 'entry > PaxExtension < 'entry > { # [doc = " Returns the key for this key/value pair parsed as a string."] # [doc = ""] # [doc = " May fail if the key isn't actually utf-8."] pub fn key (& self) -> Result < & 'entry str , str :: Utf8Error > { str :: from_utf8 (self . key) } # [doc = " Returns the underlying raw bytes for the key of this key/value pair."] pub fn key_bytes (& self) -> & 'entry [u8] { self . key } # [doc = " Returns the value for this key/value pair parsed as a string."] # [doc = ""] # [doc = " May fail if the value isn't actually utf-8."] pub fn value (& self) -> Result < & 'entry str , str :: Utf8Error > { str :: from_utf8 (self . value) } # [doc = " Returns the underlying raw bytes for this value of this key/value pair."] pub fn value_bytes (& self) -> & 'entry [u8] { self . value } }
};
}
