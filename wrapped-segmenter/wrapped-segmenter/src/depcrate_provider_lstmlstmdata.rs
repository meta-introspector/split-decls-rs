// Generated macro for LstmData (enum)
macro_rules! Depcrate_provider_lstmLstmData {
() => {
// Module: crate::provider::lstm
// Provides: {"LstmData"}
// Dependencies: {}
# [doc = " The data to power the LSTM segmentation model."] # [doc = ""] # [doc = " This data enum is extensible: more backends may be added in the future."] # [doc = " Old data can be used with newer code but not vice versa."] # [doc = ""] # [doc = " Examples of possible future extensions:"] # [doc = ""] # [doc = " 1. Variant to store data in 16 instead of 32 bits"] # [doc = " 2. Minor changes to the LSTM model, such as different forward/backward matrix sizes"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_segmenter :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [yoke (prove_covariance_manually)] # [non_exhaustive] pub enum LstmData < 'data > { # [doc = " The data as matrices of zerovec f32 values."] Float32 (# [cfg_attr (feature = "serde" , serde (borrow))] LstmDataFloat32 < 'data >) , }
};
}
