// Generated macro for span_lo_for_param (function)
macro_rules! Depcrate_itemsspan_lo_for_param {
() => {
// Module: crate::items
// Provides: {"span_lo_for_param"}
// Dependencies: {}
pub (crate) fn span_lo_for_param (param : & ast :: Param) -> BytePos { if param . attrs . is_empty () { if is_named_param (param) { param . pat . span . lo () } else { param . ty . span . lo () } } else { param . attrs [0] . span . lo () } }
};
}
