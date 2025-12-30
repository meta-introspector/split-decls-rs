// Generated macro for insert_all (function)
macro_rules! Depcrate_tedinsert_all {
() => {
// Module: crate::ted
// Provides: {"insert_all"}
// Dependencies: {}
pub fn insert_all (position : Position , mut elements : Vec < SyntaxElement >) { if let Some (first) = elements . first () && let Some (ws) = ws_before (& position , first) { elements . insert (0 , ws . into ()) ; } if let Some (last) = elements . last () && let Some (ws) = ws_after (& position , last) { elements . push (ws . into ()) ; } insert_all_raw (position , elements) ; }
};
}
