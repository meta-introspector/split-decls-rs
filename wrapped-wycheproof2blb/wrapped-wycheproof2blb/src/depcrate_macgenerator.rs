// Generated macro for generator (function)
macro_rules! Depcrate_macgenerator {
() => {
// Module: crate::mac
// Provides: {"generator"}
// Dependencies: {}
pub fn generator (data : & [u8] , algorithm : & str , key_size : u32) -> Vec < TestInfo > { let suite : TestSuite = serde_json :: from_slice (data) . unwrap () ; assert_eq ! (algorithm , suite . suite . algorithm) ; let mut infos = vec ! [] ; for g in & suite . test_groups { for tc in & g . tests { if key_size != 0 && g . key_size != key_size { continue ; } if tc . case . result != CaseResult :: Valid { continue ; } assert_eq ! (tc . key . len () * 8 , g . key_size as usize) ; assert_eq ! (g . tag_size % 8 , 0) ; infos . push (TestInfo { data : vec ! [tc . key . clone () , tc . msg . clone () , tc . tag . clone () ,] , desc : description (& suite . suite , & tc . case) , }) ; } } infos }
};
}
