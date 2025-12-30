// Generated macro for least_common_ancestor (function)
macro_rules! Depcrate_algoleast_common_ancestor {
() => {
// Module: crate::algo
// Provides: {"least_common_ancestor"}
// Dependencies: {}
pub fn least_common_ancestor (u : & SyntaxNode , v : & SyntaxNode) -> Option < SyntaxNode > { if u == v { return Some (u . clone ()) ; } let u_depth = u . ancestors () . count () ; let v_depth = v . ancestors () . count () ; let keep = u_depth . min (v_depth) ; let u_candidates = u . ancestors () . skip (u_depth - keep) ; let v_candidates = v . ancestors () . skip (v_depth - keep) ; let (res , _) = u_candidates . zip (v_candidates) . find (| (x , y) | x == y) ? ; Some (res) }
};
}
