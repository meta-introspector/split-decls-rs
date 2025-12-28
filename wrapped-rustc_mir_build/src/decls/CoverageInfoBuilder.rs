macro_rules! deps {
    () => {
        NotInfo!();
        BlockMarkerGen!();
        BranchInfo!();
    };
}

macro_rules! CoverageInfoBuilder {
    () => {
        deps!();
        # [doc = " Collects coverage-related information during MIR building, to eventually be"] # [doc = " turned into a function's [`CoverageInfoHi`] when MIR building is complete."] pub (crate) struct CoverageInfoBuilder { # [doc = " Maps condition expressions to their enclosing `!`, for better instrumentation."] nots : FxHashMap < ExprId , NotInfo > , markers : BlockMarkerGen , # [doc = " Present if branch coverage is enabled."] branch_info : Option < BranchInfo > , }
    };
}

CoverageInfoBuilder!();