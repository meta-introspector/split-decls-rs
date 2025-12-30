// Generated macro for BreakState (enum)
macro_rules! Depcrate_providerBreakState {
() => {
// Module: crate::provider
// Provides: {"BreakState"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq , Debug)] # [cfg_attr (feature = "datagen" , derive (databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_segmenter :: provider))] # [doc = " Break state"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. In particular, the `DataProvider` implementations are only"] # [doc = " guaranteed to match with this version's `*_unstable` providers. Use with caution."] # [doc = " </div>"] pub enum BreakState { # [doc = " Break"] Break , # [doc = " Keep rule"] Keep , # [doc = " Non-matching rule"] NoMatch , # [doc = " We have to look ahead one more character."] Intermediate (u8) , # [doc = " Index of a state."] Index (u8) , }
};
}
