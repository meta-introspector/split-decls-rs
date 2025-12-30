// Generated macro for parse_rgb_color (function)
macro_rules! Depcrate_ansiparse_rgb_color {
() => {
// Module: crate::ansi
// Provides: {"parse_rgb_color"}
// Dependencies: {}
# [doc = " Parse colors in `rgb:r(rrr)/g(ggg)/b(bbb)` format."] fn parse_rgb_color (color : & [u8]) -> Option < Rgb > { let colors = str :: from_utf8 (color) . ok () ? . split ('/') . collect :: < Vec < _ > > () ; if colors . len () != 3 { return None ; } let scale = | input : & str | { if input . len () > 4 { None } else { let max = u32 :: pow (16 , input . len () as u32) - 1 ; let value = u32 :: from_str_radix (input , 16) . ok () ? ; Some ((255 * value / max) as u8) } } ; Some (Rgb { r : scale (colors [0]) ? , g : scale (colors [1]) ? , b : scale (colors [2]) ? }) }
};
}
