// Generated macro for simulator_targets_set_env (function)
macro_rules! Depcrate_spec_base_apple_testssimulator_targets_set_env {
() => {
// Module: crate::spec::base::apple::tests
// Provides: {"simulator_targets_set_env"}
// Dependencies: {}
# [test] fn simulator_targets_set_env () { let all_sim_targets = [x86_64_apple_ios :: target () , x86_64_apple_tvos :: target () , x86_64_apple_watchos_sim :: target () , aarch64_apple_ios_sim :: target () , aarch64_apple_watchos_sim :: target () , aarch64_apple_visionos_sim :: target () ,] ; for target in & all_sim_targets { assert_eq ! (target . env , "sim") ; assert_eq ! (target . abi , "sim") ; } }
};
}
