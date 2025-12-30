// Generated macro for bench_for_each_provider (macro)
macro_rules! Depcrate_test_macrosbench_for_each_provider {
() => {
// Module: crate::test_macros
// Provides: {"bench_for_each_provider"}
// Dependencies: {}
# [doc = " Instantiate the given benchmark functions once for each built-in provider."] # [doc = ""] # [doc = " The selected provider module is bound as `provider`; you can rely on this"] # [doc = " having the union of the items common to the `crypto::ring` and"] # [doc = " `crypto::aws_lc_rs` modules."] # [cfg (bench)] macro_rules ! bench_for_each_provider { ($ ($ tt : tt) +) => { # [cfg (feature = "ring")] mod bench_with_ring { use crate :: crypto :: ring as provider ; # [allow (unused_imports)] use super ::*; $ ($ tt) + } # [cfg (feature = "aws-lc-rs")] mod bench_with_aws_lc_rs { use crate :: crypto :: aws_lc_rs as provider ; # [allow (unused_imports)] use super ::*; $ ($ tt) + } } ; }
};
}
