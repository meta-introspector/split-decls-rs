// Generated macro for generator (function)
macro_rules! Depcrate_aes_sivgenerator {
() => {
// Module: crate::aes_siv
// Provides: {"generator"}
// Dependencies: {}
pub fn generator (data : & [u8] , algorithm : & str , key_size : u32) -> Vec < TestInfo > { let suite : TestSuite = serde_json :: from_slice (data) . unwrap () ; assert_eq ! (algorithm , suite . suite . algorithm) ; let mut infos = vec ! [] ; for g in & suite . test_groups { if key_size != 0 && g . key_size != key_size { continue ; } for tc in & g . tests { infos . push (TestInfo { data : vec ! [tc . key . clone () , tc . aad . clone () , tc . msg . clone () , tc . ct . clone () , vec ! [case_result (& tc . case)] ,] , desc : description (& suite . suite , & tc . case) , }) ; } } infos }
};
}
