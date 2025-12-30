// Generated macro for use_56 (pub_use)
macro_rules! Depcrateuse_56 {
() => {
// Module: crate
// Provides: {"use_56"}
// Dependencies: {}
# [doc = " Convenience macro to declare a static public key and functions to interact with it."] # [doc = ""] # [doc = " Input: a single literal base58 string representation of a program's id"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # // wrapper is used so that the macro invocation occurs in the item position"] # [doc = " # // rather than in the statement position which isn't allowed."] # [doc = " use std::str::FromStr;"] # [doc = " use solana_sdk::{declare_id, pubkey::Pubkey};"] # [doc = ""] # [doc = " # mod item_wrapper {"] # [doc = " #   use solana_sdk::declare_id;"] # [doc = " declare_id!(\"My11111111111111111111111111111111111111111\");"] # [doc = " # }"] # [doc = " # use item_wrapper::id;"] # [doc = ""] # [doc = " let my_id = Pubkey::from_str(\"My11111111111111111111111111111111111111111\").unwrap();"] # [doc = " assert_eq!(id(), my_id);"] # [doc = " ```"] pub use solana_sdk_macro :: declare_id ;
};
}
