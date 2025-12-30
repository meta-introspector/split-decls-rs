// Generated macro for generator (function)
macro_rules! Depcrate_aeadgenerator {
() => {
// Module: crate::aead
// Provides: {"generator"}
// Dependencies: {}
fn generator (data : & [u8] , algorithm : & str , key_size : u32 , iv_size : u32) -> Vec < TestInfo > { let suite : TestSuite = serde_json :: from_slice (data) . unwrap () ; assert_eq ! (algorithm , suite . suite . algorithm) ; let mut infos = vec ! [] ; for g in & suite . test_groups { for tc in & g . tests { if key_size != 0 && g . key_size != key_size { continue ; } if g . iv_size != iv_size { println ! (" skipping tests for iv_size={}" , g . iv_size) ; continue ; } let mut combined_ct = Vec :: new () ; combined_ct . extend_from_slice (& tc . ct) ; combined_ct . extend_from_slice (& tc . tag) ; infos . push (TestInfo { data : vec ! [tc . key . clone () , tc . iv . clone () , tc . aad . clone () , tc . msg . clone () , combined_ct , vec ! [case_result (& tc . case)] ,] , desc : description (& suite . suite , & tc . case) , }) ; } } infos }
};
}
