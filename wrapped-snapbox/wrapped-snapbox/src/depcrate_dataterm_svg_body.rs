// Generated macro for term_svg_body (function)
macro_rules! Depcrate_dataterm_svg_body {
() => {
// Module: crate::data
// Provides: {"term_svg_body"}
// Dependencies: {}
# [cfg (feature = "term-svg")] fn term_svg_body (svg : & str) -> Option < & str > { let (_header , body , _footer) = split_term_svg (svg) ? ; Some (body) }
};
}
