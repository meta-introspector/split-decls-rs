// Generated macro for detect_feature (macro)
macro_rules! Depcrate_detect_macrosdetect_feature {
() => {
// Module: crate::detect::macros
// Provides: {"detect_feature"}
// Dependencies: {}
# [macro_export] # [allow_internal_unstable (stdarch_internal)] # [unstable (feature = "stdarch_internal" , issue = "none")] macro_rules ! detect_feature { ($ feature : tt , $ feature_lit : tt) => { $ crate :: detect_feature ! ($ feature , $ feature_lit : $ feature_lit) } ; ($ feature : tt , $ feature_lit : tt : $ ($ target_feature_lit : tt) ,*) => { $ (cfg ! (target_feature = $ target_feature_lit) ||) * $ crate :: detect :: __is_feature_detected ::$ feature () } ; ($ feature : tt , $ feature_lit : tt , without cfg check : true) => { $ crate :: detect :: __is_feature_detected ::$ feature () } ; }
};
}
