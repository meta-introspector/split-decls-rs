// Generated macro for WipEvaluationStep (struct)
macro_rules! Depcrate_solve_inspect_buildWipEvaluationStep {
() => {
// Module: crate::solve::inspect::build
// Provides: {"WipEvaluationStep"}
// Dependencies: {}
# [derive_where (PartialEq , Eq , Debug ; I : Interner)] struct WipEvaluationStep < I : Interner > { # [doc = " Unlike `EvalCtxt::var_values`, we append a new"] # [doc = " generic arg here whenever we create a new inference"] # [doc = " variable."] # [doc = ""] # [doc = " This is necessary as we otherwise don't unify these"] # [doc = " vars when instantiating multiple `CanonicalState`."] var_values : Vec < I :: GenericArg > , probe_depth : usize , evaluation : WipProbe < I > , }
};
}
