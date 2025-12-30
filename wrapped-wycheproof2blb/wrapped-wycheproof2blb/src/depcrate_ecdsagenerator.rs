// Generated macro for generator (function)
macro_rules! Depcrate_ecdsagenerator {
() => {
// Module: crate::ecdsa
// Provides: {"generator"}
// Dependencies: {}
pub fn generator (data : & [u8] , algorithm : & str , _key_size : u32) -> Vec < TestInfo > { let suite : TestSuite = serde_json :: from_slice (data) . unwrap () ; let mut infos = vec ! [] ; for g in & suite . test_groups { assert ! (algorithm . starts_with (& g . key . curve)) ; assert ! (matches ! (g . sha . as_str () , "SHA-224" | "SHA-256" | "SHA-384" | "SHA-512")) ; for tc in & g . tests { if tc . case . result == crate :: wycheproof :: CaseResult :: Acceptable { continue ; } infos . push (TestInfo { data : vec ! [g . key . wx . clone () , g . key . wy . clone () , tc . msg . clone () , tc . sig . clone () , vec ! [case_result (& tc . case)] ,] , desc : description (& suite . suite , & tc . case) , }) ; } } infos }
};
}
