// Generated macro for maybe_valid_chars (function)
macro_rules! Depcrate_identifiermaybe_valid_chars {
() => {
// Module: crate::identifier
// Provides: {"maybe_valid_chars"}
// Dependencies: {}
fn maybe_valid_chars (name : & str) -> impl Iterator < Item = Option < char > > + '_ { let mut chars = name . chars () ; core :: iter :: once (chars . next () . filter (| & c | is_id_start (c))) . chain (chars . map (| c | { if is_id_continue (c) { Some (c) } else { None } })) }
};
}
