// Generated macro for unique_signers (function)
macro_rules! Depcrateunique_signers {
() => {
// Module: crate
// Provides: {"unique_signers"}
// Dependencies: {}
# [doc = " Removes duplicate signers while preserving order. O(n²)"] pub fn unique_signers (signers : Vec < & dyn Signer >) -> Vec < & dyn Signer > { let capacity = signers . len () ; let mut out = Vec :: with_capacity (capacity) ; let mut seen = std :: collections :: HashSet :: with_capacity (capacity) ; for signer in signers { let pubkey = signer . pubkey () ; if ! seen . contains (& pubkey) { seen . insert (pubkey) ; out . push (signer) ; } } out }
};
}
