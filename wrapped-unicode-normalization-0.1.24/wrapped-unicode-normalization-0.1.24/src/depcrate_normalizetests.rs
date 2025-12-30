// Generated macro for tests (module)
macro_rules! Depcrate_normalizetests {
() => {
// Module: crate::normalize
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: compose_hangul ; # [test] fn test_hangul_composition () { assert_eq ! (compose_hangul ('\u{c8e0}' , '\u{11a7}') , None) ; } }
};
}
