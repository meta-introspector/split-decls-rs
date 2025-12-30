// Generated macro for config_changed (function)
macro_rules! Depcrate_contextconfig_changed {
() => {
// Module: crate::context
// Provides: {"config_changed"}
// Dependencies: {}
# [doc = " The solver configuration has changed."] pub fn config_changed (mut ctx : partial ! (Context , mut VsidsP , mut ClauseActivityP , SolverConfigP) , _update : & SolverConfigUpdate ,) { let (config , mut ctx) = ctx . split_part (SolverConfigP) ; ctx . part_mut (VsidsP) . set_decay (config . vsids_decay) ; ctx . part_mut (ClauseActivityP) . set_decay (config . clause_activity_decay) ; }
};
}
