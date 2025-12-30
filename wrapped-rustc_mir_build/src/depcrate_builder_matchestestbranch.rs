// Generated macro for TestBranch (enum)
macro_rules! Depcrate_builder_matchesTestBranch {
() => {
// Module: crate::builder::matches
// Provides: {"TestBranch"}
// Dependencies: {}
# [doc = " The branch to be taken after a test."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] enum TestBranch < 'tcx > { # [doc = " Success branch, used for tests with two possible outcomes."] Success , # [doc = " Branch corresponding to this constant. Must be a scalar."] Constant (ty :: Value < 'tcx >) , # [doc = " Branch corresponding to this variant."] Variant (VariantIdx) , # [doc = " Failure branch for tests with two possible outcomes, and \"otherwise\" branch for other tests."] Failure , }
};
}
