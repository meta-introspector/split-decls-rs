// Generated macro for FIXPOINT_STEP_LIMIT (const)
macro_rules! Depcrate_solveFIXPOINT_STEP_LIMIT {
() => {
// Module: crate::solve
// Provides: {"FIXPOINT_STEP_LIMIT"}
// Dependencies: {}
# [doc = " How many fixpoint iterations we should attempt inside of the solver before bailing"] # [doc = " with overflow."] # [doc = ""] # [doc = " We previously used  `cx.recursion_limit().0.checked_ilog2().unwrap_or(0)` for this."] # [doc = " However, it feels unlikely that uncreasing the recursion limit by a power of two"] # [doc = " to get one more itereation is every useful or desirable. We now instead used a constant"] # [doc = " here. If there ever ends up some use-cases where a bigger number of fixpoint iterations"] # [doc = " is required, we can add a new attribute for that or revert this to be dependant on the"] # [doc = " recursion limit again. However, this feels very unlikely."] const FIXPOINT_STEP_LIMIT : usize = 8 ;
};
}
