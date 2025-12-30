// Generated macro for equivalent_key (function)
macro_rules! Depcrate_query_plumbingequivalent_key {
() => {
// Module: crate::query::plumbing
// Provides: {"equivalent_key"}
// Dependencies: {}
# [inline] fn equivalent_key < K : Eq , V > (k : & K) -> impl Fn (& (K , V)) -> bool + '_ { move | x | x . 0 == * k }
};
}
