// Generated macro for split_term_svg (function)
macro_rules! Depcrate_datasplit_term_svg {
() => {
// Module: crate::data
// Provides: {"split_term_svg"}
// Dependencies: {}
# [cfg (feature = "term-svg")] pub (crate) fn split_term_svg (svg : & str) -> Option < (& str , & str , & str) > { let open_elem_start_idx = svg . find ("<text") ? ; _ = svg [open_elem_start_idx ..] . find ('>') ? ; let open_elem_line_start_idx = svg [.. open_elem_start_idx] . rfind ('\n') . map (| idx | idx + 1) . unwrap_or (svg . len ()) ; let close_elem = "</text>" ; let close_elem_start_idx = svg . rfind (close_elem) . unwrap_or (svg . len ()) ; let close_elem_line_end_idx = svg [close_elem_start_idx ..] . find ('\n') . map (| idx | idx + close_elem_start_idx + 1) . unwrap_or (svg . len ()) ; let header = & svg [.. open_elem_line_start_idx] ; let body = & svg [open_elem_line_start_idx .. close_elem_line_end_idx] ; let footer = & svg [close_elem_line_end_idx ..] ; Some ((header , body , footer)) }
};
}
