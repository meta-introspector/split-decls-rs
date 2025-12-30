// Generated macro for encode_path (function)
macro_rules! Depcrate_utilencode_path {
() => {
// Module: crate::util
// Provides: {"encode_path"}
// Dependencies: {}
# [cfg (windows)] pub fn encode_path (dst : & mut dyn Write , path : & Path) -> std :: io :: Result < () > { use std :: os :: windows :: prelude :: * ; let points = path . as_os_str () . encode_wide () . collect :: < Vec < _ > > () ; let bytes = wide_char_to_multi_byte (& points) ? ; dst . write_all (& bytes) }
};
}
