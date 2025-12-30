// Generated macro for dummy (module)
macro_rules! Depcrate_tests_ffidummy {
() => {
// Module: crate::tests::ffi
// Provides: {"dummy"}
// Dependencies: {}
# [cfg (not (target_os = "android"))] mod dummy { # ! [doc = " A module to prevent dead-code warnings all over"] # ! [doc = " the `tests` module due to the weird combination of"] # ! [doc = " feature flags and `--all-features`. These test case"] # ! [doc = " lists are only used via the FFI."] use crate :: tests ; # [allow (dead_code)] fn dummy () { # [cfg (any (windows , target_os = "android" , target_vendor = "apple" , target_os = "linux"))] let _ = tests :: verification_mock :: ALL_TEST_CASES ; let _ = tests :: verification_real_world :: ALL_TEST_CASES ; } }
};
}
