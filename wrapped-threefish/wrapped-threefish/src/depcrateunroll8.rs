// Generated macro for unroll8 (macro)
macro_rules! Depcrateunroll8 {
() => {
// Module: crate
// Provides: {"unroll8"}
// Dependencies: {}
# [cfg (feature = "no_unroll")] macro_rules ! unroll8 { ($ var : ident , $ body : block) => { for $ var in 0 .. 8 $ body } }
};
}
