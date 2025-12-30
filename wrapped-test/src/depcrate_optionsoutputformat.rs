// Generated macro for OutputFormat (enum)
macro_rules! Depcrate_optionsOutputFormat {
() => {
// Module: crate::options
// Provides: {"OutputFormat"}
// Dependencies: {}
# [doc = " Format of the test results output"] # [derive (Copy , Clone , Debug , Default , PartialEq , Eq)] pub enum OutputFormat { # [doc = " Verbose output"] Pretty , # [doc = " Quiet output"] # [default] Terse , # [doc = " JSON output"] Json , # [doc = " JUnit output"] Junit , }
};
}
