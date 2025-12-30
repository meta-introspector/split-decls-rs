// Generated macro for fill_background (function)
macro_rules! Depcratefill_background {
() => {
// Module: crate
// Provides: {"fill_background"}
// Dependencies: {}
# [doc = " Helper method to quickly set the background of all cells inside the specified area."] pub fn fill_background (area : Rect , buf : & mut Buffer , color : Color) { for y in area . top () .. area . bottom () { for x in area . left () .. area . right () { buf . get_mut (x , y) . set_bg (color) ; } } }
};
}
