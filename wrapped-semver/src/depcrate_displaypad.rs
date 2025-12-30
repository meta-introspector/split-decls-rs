// Generated macro for pad (function)
macro_rules! Depcrate_displaypad {
() => {
// Module: crate::display
// Provides: {"pad"}
// Dependencies: {}
fn pad (formatter : & mut fmt :: Formatter , do_display : impl FnOnce (& mut fmt :: Formatter) -> fmt :: Result , do_len : impl FnOnce () -> usize ,) -> fmt :: Result { let min_width = match formatter . width () { Some (min_width) => min_width , None => return do_display (formatter) , } ; let len = do_len () ; if len >= min_width { return do_display (formatter) ; } let default_align = Alignment :: Left ; let align = formatter . align () . unwrap_or (default_align) ; let padding = min_width - len ; let (pre_pad , post_pad) = match align { Alignment :: Left => (0 , padding) , Alignment :: Right => (padding , 0) , Alignment :: Center => (padding / 2 , (padding + 1) / 2) , } ; let fill = formatter . fill () ; for _ in 0 .. pre_pad { formatter . write_char (fill) ? ; } do_display (formatter) ? ; for _ in 0 .. post_pad { formatter . write_char (fill) ? ; } Ok (()) }
};
}
