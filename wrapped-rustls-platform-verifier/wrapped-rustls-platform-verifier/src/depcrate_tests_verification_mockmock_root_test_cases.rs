// Generated macro for mock_root_test_cases (macro)
macro_rules! Depcrate_tests_verification_mockmock_root_test_cases {
() => {
// Module: crate::tests::verification_mock
// Provides: {"mock_root_test_cases"}
// Dependencies: {}
macro_rules ! mock_root_test_cases { { $ ($ name : ident [$ target : meta] => $ test_case : expr) ,+ , } => { mock_root_test_cases ! (@ $ ($ name [$ target] => $ test_case) ,+,) ; # [cfg (test)] mod tests { $ (# [cfg ($ target)] # [test] pub fn $ name () { super ::$ name () }) + } # [cfg (feature = "ffi-testing")] pub static ALL_TEST_CASES : &'static [fn ()] = & [$ (# [cfg ($ target)] $ name ,) +] ; } ; { @ $ ($ name : ident [$ target : meta] => $ test_case : expr) ,+ , } => { $ (# [cfg ($ target)] pub (super) fn $ name () { test_with_mock_root (&$ test_case , Roots :: OnlyExtra) ; # [cfg (all ($ target , not (target_os = "android")))] test_with_mock_root (&$ test_case , Roots :: ExtraAndPlatform) ; }) + } ; }
};
}
