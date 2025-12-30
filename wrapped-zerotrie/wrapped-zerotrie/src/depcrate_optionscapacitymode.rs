// Generated macro for CapacityMode (enum)
macro_rules! Depcrate_optionsCapacityMode {
() => {
// Module: crate::options
// Provides: {"CapacityMode"}
// Dependencies: {}
# [doc = " Whether to enforce a limit to the capacity of the ZeroTrie."] # [derive (Copy , Clone)] pub (crate) enum CapacityMode { # [doc = " Return an error if the trie requires a branch of more than 2^32 bytes."] Normal , # [doc = " Construct the trie without returning an error."] Extended , }
};
}
