// Generated macro for tests (module)
macro_rules! Depcrate_timestamptests {
() => {
// Module: crate::timestamp
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , any (feature = "v1" , feature = "v6")))] mod tests { use super :: * ; # [cfg (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none")))] use wasm_bindgen_test :: * ; # [test] # [cfg_attr (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none")) , wasm_bindgen_test)] fn gregorian_unix_does_not_panic () { Timestamp :: unix_to_gregorian_ticks (u64 :: MAX , 0) ; Timestamp :: unix_to_gregorian_ticks (0 , u32 :: MAX) ; Timestamp :: unix_to_gregorian_ticks (u64 :: MAX , u32 :: MAX) ; Timestamp :: gregorian_to_unix (u64 :: MAX) ; } # [test] # [cfg_attr (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none")) , wasm_bindgen_test)] fn to_gregorian_truncates_to_usable_bits () { let ts = Timestamp :: from_gregorian (123 , u16 :: MAX) ; assert_eq ! ((123 , u16 :: MAX >> 2) , ts . to_gregorian ()) ; } }
};
}
