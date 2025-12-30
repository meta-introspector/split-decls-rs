// Generated macro for cmd_coverage_lcov (function)
macro_rules! Depcratecmd_coverage_lcov {
() => {
// Module: crate
// Provides: {"cmd_coverage_lcov"}
// Dependencies: {}
fn cmd_coverage_lcov () -> Result < () , DynError > { cargo (& ["tarpaulin" , "--features" , "all" , "--ignore-tests" , "--out" , "Lcov" ,]) }
};
}
