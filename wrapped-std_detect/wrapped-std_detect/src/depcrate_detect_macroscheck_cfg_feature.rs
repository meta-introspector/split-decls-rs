// Generated macro for check_cfg_feature (macro)
macro_rules! Depcrate_detect_macroscheck_cfg_feature {
() => {
// Module: crate::detect::macros
// Provides: {"check_cfg_feature"}
// Dependencies: {}
# [allow (unused_macros , reason = "it's used in the features! macro below")] macro_rules ! check_cfg_feature { ($ feature : tt , $ feature_lit : tt) => { check_cfg_feature ! ($ feature , $ feature_lit : $ feature_lit) } ; ($ feature : tt , $ feature_lit : tt : $ ($ target_feature_lit : tt) ,*) => { $ (cfg ! (target_feature = $ target_feature_lit) ;) * } ; ($ feature : tt , $ feature_lit : tt , without cfg check : $ feature_cfg_check : literal) => { # [allow (unexpected_cfgs , reason = $ feature_lit)] { cfg ! (target_feature = $ feature_lit) } } ; }
};
}
