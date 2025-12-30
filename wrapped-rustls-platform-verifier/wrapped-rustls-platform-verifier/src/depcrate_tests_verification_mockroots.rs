// Generated macro for Roots (enum)
macro_rules! Depcrate_tests_verification_mockRoots {
() => {
// Module: crate::tests::verification_mock
// Provides: {"Roots"}
// Dependencies: {}
enum Roots { # [doc = " Test with only extra roots, without loading the platform trust store."] # [doc = ""] # [doc = " We want to keep things reproducible given the background-managed nature of trust roots on platforms."] OnlyExtra , # [doc = " Test with loading the extra roots and the platform trust store."] # [doc = ""] # [doc = " Right now, not all platforms are supported."] # [cfg (not (target_os = "android"))] ExtraAndPlatform , }
};
}
