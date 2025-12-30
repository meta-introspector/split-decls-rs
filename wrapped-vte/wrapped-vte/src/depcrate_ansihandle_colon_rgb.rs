// Generated macro for handle_colon_rgb (function)
macro_rules! Depcrate_ansihandle_colon_rgb {
() => {
// Module: crate::ansi
// Provides: {"handle_colon_rgb"}
// Dependencies: {}
# [doc = " Handle colon separated rgb color escape sequence."] # [inline] fn handle_colon_rgb (params : & [u16]) -> Option < Color > { let rgb_start = if params . len () > 4 { 2 } else { 1 } ; let rgb_iter = params [rgb_start ..] . iter () . copied () ; let mut iter = iter :: once (params [0]) . chain (rgb_iter) ; parse_sgr_color (& mut iter) }
};
}
