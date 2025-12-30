// Generated macro for unroll8_rev (macro)
macro_rules! Depcrateunroll8_rev {
() => {
// Module: crate
// Provides: {"unroll8_rev"}
// Dependencies: {}
# [cfg (feature = "no_unroll")] macro_rules ! unroll8_rev { ($ var : ident , $ body : block) => { for $ var in (0 .. 8) . rev () $ body } }
};
}
