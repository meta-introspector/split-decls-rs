// Generated macro for Assumptions (struct)
macro_rules! Depcrate_assumptionsAssumptions {
() => {
// Module: crate::assumptions
// Provides: {"Assumptions"}
// Dependencies: {}
# [doc = " Incremental solving."] # [derive (Default)] pub struct Assumptions { assumptions : Vec < Lit > , failed_core : Vec < Lit > , user_failed_core : Vec < Lit > , assumption_levels : usize , failed_propagation_hashes : Vec < ClauseHash > , }
};
}
