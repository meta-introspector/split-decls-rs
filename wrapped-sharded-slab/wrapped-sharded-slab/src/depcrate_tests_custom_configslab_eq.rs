// Generated macro for slab_eq (function)
macro_rules! Depcrate_tests_custom_configslab_eq {
() => {
// Module: crate::tests::custom_config
// Provides: {"slab_eq"}
// Dependencies: {}
# [track_caller] fn slab_eq (mut lhs : Slab < u64 , impl Config > , mut rhs : Slab < u64 , impl Config >) { let mut lhs_vec = lhs . unique_iter () . collect :: < Vec < _ > > () ; lhs_vec . sort_unstable () ; let mut rhs_vec = rhs . unique_iter () . collect :: < Vec < _ > > () ; rhs_vec . sort_unstable () ; assert_eq ! (lhs_vec , rhs_vec) ; }
};
}
