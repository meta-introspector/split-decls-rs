// Generated macro for generate_json (function)
macro_rules! Depcrategenerate_json {
() => {
// Module: crate
// Provides: {"generate_json"}
// Dependencies: {}
pub fn generate_json () -> String { let mut cases = Vec :: new () ; for & input_len in TEST_CASES { let mut input = vec ! [0 ; input_len] ; paint_test_input (& mut input) ; let mut hash_out = [0 ; OUTPUT_LEN] ; blake3 :: Hasher :: new () . update (& input) . finalize_xof () . fill (& mut hash_out) ; let mut keyed_hash_out = [0 ; OUTPUT_LEN] ; blake3 :: Hasher :: new_keyed (TEST_KEY) . update (& input) . finalize_xof () . fill (& mut keyed_hash_out) ; let mut derive_key_out = [0 ; OUTPUT_LEN] ; blake3 :: Hasher :: new_derive_key (TEST_CONTEXT) . update (& input) . finalize_xof () . fill (& mut derive_key_out) ; cases . push (Case { input_len , hash : hex :: encode (& hash_out [..]) , keyed_hash : hex :: encode (& keyed_hash_out [..]) , derive_key : hex :: encode (& derive_key_out [..]) , }) ; } let mut json = serde_json :: to_string_pretty (& Cases { _comment : COMMENT . trim () . replace ("\n" , " ") , key : std :: str :: from_utf8 (TEST_KEY) . unwrap () . to_string () , context_string : TEST_CONTEXT . to_string () , cases , }) . unwrap () ; json . push ('\n') ; json }
};
}
