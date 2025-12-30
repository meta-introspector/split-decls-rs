// Generated macro for xparse_color (function)
macro_rules! Depcrate_ansixparse_color {
() => {
// Module: crate::ansi
// Provides: {"xparse_color"}
// Dependencies: {}
# [doc = " Parse colors in XParseColor format."] fn xparse_color (color : & [u8]) -> Option < Rgb > { if ! color . is_empty () && color [0] == b'#' { parse_legacy_color (& color [1 ..]) } else if color . len () >= 4 && & color [.. 4] == b"rgb:" { parse_rgb_color (& color [4 ..]) } else { None } }
};
}
