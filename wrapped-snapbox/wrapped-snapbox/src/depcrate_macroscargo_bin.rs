// Generated macro for cargo_bin (macro)
macro_rules! Depcrate_macroscargo_bin {
() => {
// Module: crate::macros
// Provides: {"cargo_bin"}
// Dependencies: {}
# [doc = " The absolute path to a binary target's executable."] # [doc = ""] # [doc = " The `bin_target_name` is the name of the binary"] # [doc = " target, exactly as-is."] # [doc = ""] # [doc = " **NOTE:** This is only set when building an integration test or benchmark."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " #[test]"] # [doc = " fn cli_tests() {"] # [doc = "     trycmd::TestCases::new()"] # [doc = "         .default_bin_path(trycmd::cargo_bin!(\"bin-fixture\"))"] # [doc = "         .case(\"tests/cmd/*.trycmd\");"] # [doc = " }"] # [doc = " ```"] # [macro_export] # [doc (hidden)] macro_rules ! cargo_bin { () => { $ crate :: cmd :: cargo_bin ! (env ! ("CARGO_PKG_NAME")) } ; ($ bin_target_name : expr) => { :: std :: path :: Path :: new (env ! (concat ! ("CARGO_BIN_EXE_" , $ bin_target_name))) } ; }
};
}
