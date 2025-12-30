// Generated macro for FlagType (enum)
macro_rules! Depcrate_cmdlineFlagType {
() => {
// Module: crate::cmdline
// Provides: {"FlagType"}
// Dependencies: {}
# [doc = " How a hyphen-prefixed argument passed to the parent process should be"] # [doc = " handled when constructing the command-line for the child process."] # [derive (Clone , Copy , Debug , PartialEq)] enum FlagType { # [doc = " Pass the flag through unchanged. The boolean indicates whether the flag"] # [doc = " is followed by an argument."] Pass (bool) , # [doc = " Drop the flag entirely. The boolean indicates whether the flag is"] # [doc = " followed by an argument."] Drop (bool) , # [doc = " Indicates a known flag that should never be encountered. The string is"] # [doc = " a human-readable error message."] Error (& 'static str) , }
};
}
