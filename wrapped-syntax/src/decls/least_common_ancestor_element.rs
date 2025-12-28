macro_rules! deps {
    () => {
        Element!();
        SyntaxNode!();
    };
}

macro_rules! least_common_ancestor_element {
    () => {
        deps!();
        pub fn least_common_ancestor_element (u : impl Element , v : impl Element) -> Option < SyntaxNode > { let u = u . syntax_element () ; let v = v . syntax_element () ; if u == v { return match u { NodeOrToken :: Node (node) => Some (node) , NodeOrToken :: Token (token) => token . parent () , } ; } let u_depth = u . ancestors () . count () ; let v_depth = v . ancestors () . count () ; let keep = u_depth . min (v_depth) ; let u_candidates = u . ancestors () . skip (u_depth - keep) ; let v_candidates = v . ancestors () . skip (v_depth - keep) ; let (res , _) = u_candidates . zip (v_candidates) . find (| (x , y) | x == y) ? ; Some (res) }
    };
}

least_common_ancestor_element!();