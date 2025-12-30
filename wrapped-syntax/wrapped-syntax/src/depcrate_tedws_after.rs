// Generated macro for ws_after (function)
macro_rules! Depcrate_tedws_after {
() => {
// Module: crate::ted
// Provides: {"ws_after"}
// Dependencies: {}
fn ws_after (position : & Position , new : & SyntaxElement) -> Option < SyntaxToken > { let next = match & position . repr { PositionRepr :: FirstChild (parent) => parent . first_child_or_token () ? , PositionRepr :: After (sibling) => sibling . next_sibling_or_token () ? , } ; ws_between (new , & next) }
};
}
