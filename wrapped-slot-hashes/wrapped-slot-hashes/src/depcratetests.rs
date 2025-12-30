// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , solana_sha256_hasher :: hash } ; # [test] fn test () { let mut slot_hashes = SlotHashes :: new (& [(1 , Hash :: default ()) , (3 , Hash :: default ())]) ; slot_hashes . add (2 , Hash :: default ()) ; assert_eq ! (slot_hashes , SlotHashes (vec ! [(3 , Hash :: default ()) , (2 , Hash :: default ()) , (1 , Hash :: default ()) ,])) ; let mut slot_hashes = SlotHashes :: new (& []) ; for i in 0 .. MAX_ENTRIES + 1 { slot_hashes . add (i as u64 , hash (& [(i >> 24) as u8 , (i >> 16) as u8 , (i >> 8) as u8 , i as u8]) ,) ; } for i in 0 .. MAX_ENTRIES { assert_eq ! (slot_hashes [i] . 0 , (MAX_ENTRIES - i) as u64) ; } assert_eq ! (slot_hashes . len () , MAX_ENTRIES) ; } }
};
}
