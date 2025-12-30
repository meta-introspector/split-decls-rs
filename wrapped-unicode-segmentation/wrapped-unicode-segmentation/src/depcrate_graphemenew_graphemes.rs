// Generated macro for new_graphemes (function)
macro_rules! Depcrate_graphemenew_graphemes {
() => {
// Module: crate::grapheme
// Provides: {"new_graphemes"}
// Dependencies: {}
# [inline] pub fn new_graphemes (s : & str , is_extended : bool) -> Graphemes < '_ > { let len = s . len () ; Graphemes { string : s , cursor : GraphemeCursor :: new (0 , len , is_extended) , cursor_back : GraphemeCursor :: new (len , len , is_extended) , } }
};
}
