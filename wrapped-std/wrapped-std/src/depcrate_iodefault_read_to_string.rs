// Generated macro for default_read_to_string (function)
macro_rules! Depcrate_iodefault_read_to_string {
() => {
// Module: crate::io
// Provides: {"default_read_to_string"}
// Dependencies: {}
pub (crate) fn default_read_to_string < R : Read + ? Sized > (r : & mut R , buf : & mut String , size_hint : Option < usize > ,) -> Result < usize > { unsafe { append_to_string (buf , | b | default_read_to_end (r , b , size_hint)) } }
};
}
