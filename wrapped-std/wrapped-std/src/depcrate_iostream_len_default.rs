// Generated macro for stream_len_default (function)
macro_rules! Depcrate_iostream_len_default {
() => {
// Module: crate::io
// Provides: {"stream_len_default"}
// Dependencies: {}
pub (crate) fn stream_len_default < T : Seek + ? Sized > (self_ : & mut T) -> Result < u64 > { let old_pos = self_ . stream_position () ? ; let len = self_ . seek (SeekFrom :: End (0)) ? ; if old_pos != len { self_ . seek (SeekFrom :: Start (old_pos)) ? ; } Ok (len) }
};
}
