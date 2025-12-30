// Generated macro for neighbor (function)
macro_rules! Depcrate_algoneighbor {
() => {
// Module: crate::algo
// Provides: {"neighbor"}
// Dependencies: {}
pub fn neighbor < T : AstNode > (me : & T , direction : Direction) -> Option < T > { me . syntax () . siblings (direction) . skip (1) . find_map (T :: cast) }
};
}
