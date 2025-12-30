// Generated macro for generator (function)
macro_rules! Depcrate_ed25519generator {
() => {
// Module: crate::ed25519
// Provides: {"generator"}
// Dependencies: {}
pub fn generator (data : & [u8] , algorithm : & str , _key_size : u32) -> Vec < TestInfo > { let suite : TestSuite = serde_json :: from_slice (data) . unwrap () ; assert_eq ! (algorithm , suite . suite . algorithm) ; let mut infos = vec ! [] ; for g in & suite . test_groups { for tc in & g . tests { infos . push (TestInfo { data : vec ! [g . key . sk . clone () , g . key . pk . clone () , tc . msg . clone () , tc . sig . clone () , vec ! [case_result (& tc . case)] ,] , desc : description (& suite . suite , & tc . case) , }) ; } } infos }
};
}
