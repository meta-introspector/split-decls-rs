// Generated macro for UCharDictionaryBreakData (struct)
macro_rules! Depcrate_providerUCharDictionaryBreakData {
() => {
// Module: crate::provider
// Provides: {"UCharDictionaryBreakData"}
// Dependencies: {}
# [doc = " char16trie data for dictionary break"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_segmenter :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct UCharDictionaryBreakData < 'data > { # [doc = " Dictionary data of char16trie."] # [cfg_attr (feature = "serde" , serde (borrow))] pub trie_data : ZeroVec < 'data , u16 > , }
};
}
