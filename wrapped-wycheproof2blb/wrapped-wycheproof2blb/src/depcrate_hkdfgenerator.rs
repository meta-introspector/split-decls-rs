// Generated macro for generator (function)
macro_rules! Depcrate_hkdfgenerator {
() => {
// Module: crate::hkdf
// Provides: {"generator"}
// Dependencies: {}
pub fn generator (data : & [u8] , algorithm : & str , _key_size : u32) -> Vec < TestInfo > { let suite : TestSuite = serde_json :: from_slice (data) . unwrap () ; assert_eq ! (algorithm , suite . suite . algorithm) ; let mut infos = vec ! [] ; for g in & suite . test_groups { for tc in & g . tests { if tc . case . result != crate :: wycheproof :: CaseResult :: Valid { continue ; } if tc . okm . len () != tc . size { eprintln ! ("Skipping case {} with size={} != okm.len()={}" , tc . case . case_id , tc . size , tc . okm . len ()) ; } infos . push (TestInfo { data : vec ! [tc . ikm . clone () , tc . salt . clone () , tc . info . clone () , tc . okm . clone () ,] , desc : description (& suite . suite , & tc . case) , }) ; } } infos }
};
}
