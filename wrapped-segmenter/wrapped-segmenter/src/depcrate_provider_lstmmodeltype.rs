// Generated macro for ModelType (enum)
macro_rules! Depcrate_provider_lstmModelType {
() => {
// Module: crate::provider::lstm
// Provides: {"ModelType"}
// Dependencies: {}
# [derive (PartialEq , Debug , Clone , Copy)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_segmenter :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [doc = " The type of LSTM model"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] pub enum ModelType { # [doc = " A model working on code points"] Codepoints , # [doc = " A model working on grapheme clusters"] GraphemeClusters , }
};
}
