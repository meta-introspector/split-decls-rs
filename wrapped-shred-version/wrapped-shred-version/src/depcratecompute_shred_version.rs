// Generated macro for compute_shred_version (function)
macro_rules! Depcratecompute_shred_version {
() => {
// Module: crate
// Provides: {"compute_shred_version"}
// Dependencies: {}
pub fn compute_shred_version (genesis_hash : & Hash , hard_forks : Option < & HardForks >) -> u16 { let mut hash = Hash :: new_from_array (genesis_hash . to_bytes ()) ; if let Some (hard_forks) = hard_forks { for & (slot , count) in hard_forks . iter () { let buf = [slot . to_le_bytes () , (count as u64) . to_le_bytes ()] . concat () ; hash = hashv (& [hash . as_ref () , & buf]) ; } } version_from_hash (& hash) }
};
}
