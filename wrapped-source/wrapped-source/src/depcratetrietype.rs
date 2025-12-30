// Generated macro for TrieType (enum)
macro_rules! DepcrateTrieType {
() => {
// Module: crate
// Provides: {"TrieType"}
// Dependencies: {}
# [doc = " Specifies the trie type to use."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Default , serde :: Serialize , serde :: Deserialize)] # [non_exhaustive] enum TrieType { # [doc = " Fast tries are optimized for speed"] # [serde (rename = "fast")] Fast , # [doc = " Small tries are optimized for size"] # [serde (rename = "small")] # [default] Small , }
};
}
