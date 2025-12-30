// Generated macro for tests (module)
macro_rules! Depcrate_recent_blockhashestests {
() => {
// Module: crate::recent_blockhashes
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , solana_clock :: MAX_PROCESSING_AGE } ; # [test] # [allow (clippy :: assertions_on_constants)] fn test_sysvar_can_hold_all_active_blockhashes () { assert ! (MAX_PROCESSING_AGE <= MAX_ENTRIES) ; } # [test] fn test_size_of () { let entry = Entry :: new (& Hash :: default () , 0) ; assert_eq ! (bincode :: serialized_size (& RecentBlockhashes (vec ! [entry ; MAX_ENTRIES])) . unwrap () as usize , RecentBlockhashes :: size_of ()) ; } }
};
}
