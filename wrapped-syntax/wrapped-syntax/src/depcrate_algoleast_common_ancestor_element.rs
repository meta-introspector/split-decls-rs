// Generated macro for least_common_ancestor_element (function)
macro_rules! Depcrate_algoleast_common_ancestor_element {
() => {
// Module: crate::algo
// Provides: {"least_common_ancestor_element"}
// Dependencies: {}
pub fn least_common_ancestor_element (u : impl Element , v : impl Element) -> Option < SyntaxNode > { let u = u . syntax_element () ; let v = v . syntax_element () ; if u == v { return match u { NodeOrToken :: Node (node) => Some (node) , NodeOrToken :: Token (token) => token . parent () , } ; } let u_depth = u . ancestors () . count () ; let v_depth = v . ancestors () . count () ; let keep = u_depth . min (v_depth) ; let u_candidates = u . ancestors () . skip (u_depth - keep) ; let v_candidates = v . ancestors () . skip (v_depth - keep) ; let (res , _) = u_candidates . zip (v_candidates) . find (| (x , y) | x == y) ? ; Some (res) }
};
}
