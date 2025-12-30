// Generated macro for CoverageInfoBuilder (struct)
macro_rules! Depcrate_builder_coverageinfoCoverageInfoBuilder {
() => {
// Module: crate::builder::coverageinfo
// Provides: {"CoverageInfoBuilder"}
// Dependencies: {}
# [doc = " Collects coverage-related information during MIR building, to eventually be"] # [doc = " turned into a function's [`CoverageInfoHi`] when MIR building is complete."] pub (crate) struct CoverageInfoBuilder { # [doc = " Maps condition expressions to their enclosing `!`, for better instrumentation."] nots : FxHashMap < ExprId , NotInfo > , markers : BlockMarkerGen , # [doc = " Present if branch coverage is enabled."] branch_info : Option < BranchInfo > , }
};
}
