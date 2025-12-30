// Generated macro for real_world_test_cases (macro)
macro_rules! Depcrate_tests_verification_real_worldreal_world_test_cases {
() => {
// Module: crate::tests::verification_real_world
// Provides: {"real_world_test_cases"}
// Dependencies: {}
macro_rules ! real_world_test_cases { { $ ($ name : ident => $ test_case : expr) ,+ , } => { real_world_test_cases ! (@ $ ($ name => $ test_case) ,+,) ; # [cfg (test)] mod tests { $ (# [test] pub fn $ name () { super ::$ name () }) + } # [cfg (feature = "ffi-testing")] pub static ALL_TEST_CASES : &'static [fn ()] = & [$ ($ name) ,+] ; } ; { @ $ ($ name : ident => $ test_case : expr) ,+ , } => { $ (pub (super) fn $ name () { real_world_test (&$ test_case) ; }) + } }
};
}
