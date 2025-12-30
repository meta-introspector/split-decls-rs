// Generated macro for use_53 (pub_use)
macro_rules! Depcrateuse_53 {
() => {
// Module: crate
// Provides: {"use_53"}
// Dependencies: {}
# [deprecated (since = "2.1.0" , note = "Use `solana_pubkey::pubkey` instead")] # [doc = " Convenience macro to define a static public key."] # [doc = ""] # [doc = " Input: a single literal base58 string representation of a Pubkey"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::str::FromStr;"] # [doc = " use solana_program::{pubkey, pubkey::Pubkey};"] # [doc = ""] # [doc = " static ID: Pubkey = pubkey!(\"My11111111111111111111111111111111111111111\");"] # [doc = ""] # [doc = " let my_id = Pubkey::from_str(\"My11111111111111111111111111111111111111111\").unwrap();"] # [doc = " assert_eq!(ID, my_id);"] # [doc = " ```"] pub use solana_pubkey :: pubkey ;
};
}
