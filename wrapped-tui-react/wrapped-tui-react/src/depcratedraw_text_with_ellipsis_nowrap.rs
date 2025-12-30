// Generated macro for draw_text_with_ellipsis_nowrap (function)
macro_rules! Depcratedraw_text_with_ellipsis_nowrap {
() => {
// Module: crate
// Provides: {"draw_text_with_ellipsis_nowrap"}
// Dependencies: {}
pub fn draw_text_with_ellipsis_nowrap (bound : Rect , buf : & mut Buffer , text : impl AsRef < str > , style : impl Into < Option < Style > > ,) -> u16 { let s = style . into () ; let t = text . as_ref () ; let mut graphemes = t . graphemes (true) ; let mut total_width = 0 ; { let mut ellipsis_candidate_x = None ; let mut x_offset = 0 ; for (g , mut x) in graphemes . by_ref () . zip (bound . left () .. bound . right ()) { let width = g . width () ; total_width += width ; x += x_offset ; let cell = buf . get_mut (x , bound . y) ; if x + 1 == bound . right () { ellipsis_candidate_x = Some (x) ; } cell . set_symbol (g . into ()) ; if let Some (s) = s { cell . set_style (s) ; } x_offset += width . saturating_sub (1) as u16 ; if x + x_offset >= bound . right () { break ; } let x = x as usize ; for x in x + 1 .. x + width { let i = buf . index_of (x as u16 , bound . y) ; buf . content [i] . reset () ; } } if let (Some (_) , Some (x)) = (graphemes . next () , ellipsis_candidate_x) { buf . get_mut (x , bound . y) . set_symbol ("…" . into ()) ; } } total_width as u16 }
};
}
