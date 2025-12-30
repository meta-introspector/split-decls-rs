// Generated macro for draw_text_nowrap_fn (function)
macro_rules! Depcratedraw_text_nowrap_fn {
() => {
// Module: crate
// Provides: {"draw_text_nowrap_fn"}
// Dependencies: {}
pub fn draw_text_nowrap_fn (bound : Rect , buf : & mut Buffer , t : impl AsRef < str > , mut s : impl FnMut (& str , u16 , u16) -> Style ,) { if bound . width == 0 { return ; } for (g , x) in t . as_ref () . graphemes (true) . zip (bound . left () .. bound . right ()) { let cell = buf . get_mut (x , bound . y) ; cell . set_symbol (g . into ()) ; cell . set_style (s (cell . symbol () , x , bound . y)) ; } }
};
}
