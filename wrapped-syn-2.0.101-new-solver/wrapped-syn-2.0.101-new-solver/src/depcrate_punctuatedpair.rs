// Generated macro for Pair (enum)
macro_rules! Depcrate_punctuatedPair {
() => {
// Module: crate::punctuated
// Provides: {"Pair"}
// Dependencies: {}
# [doc = " A single syntax tree node of type `T` followed by its trailing punctuation"] # [doc = " of type `P` if any."] # [doc = ""] # [doc = " Refer to the [module documentation] for details about punctuated sequences."] # [doc = ""] # [doc = " [module documentation]: self"] pub enum Pair < T , P > { Punctuated (T , P) , End (T) , }
};
}
