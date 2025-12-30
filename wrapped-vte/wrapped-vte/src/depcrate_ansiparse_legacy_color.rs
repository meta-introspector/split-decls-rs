// Generated macro for parse_legacy_color (function)
macro_rules! Depcrate_ansiparse_legacy_color {
() => {
// Module: crate::ansi
// Provides: {"parse_legacy_color"}
// Dependencies: {}
# [doc = " Parse colors in `#r(rrr)g(ggg)b(bbb)` format."] fn parse_legacy_color (color : & [u8]) -> Option < Rgb > { let item_len = color . len () / 3 ; let color_from_slice = | slice : & [u8] | { let col = usize :: from_str_radix (str :: from_utf8 (slice) . ok () ? , 16) . ok () ? << 4 ; Some ((col >> (4 * slice . len () . saturating_sub (1))) as u8) } ; Some (Rgb { r : color_from_slice (& color [0 .. item_len]) ? , g : color_from_slice (& color [item_len .. item_len * 2]) ? , b : color_from_slice (& color [item_len * 2 ..]) ? , }) }
};
}
