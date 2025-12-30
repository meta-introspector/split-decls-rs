// Generated macro for Punctuated (struct)
macro_rules! Depcrate_punctuatedPunctuated {
() => {
// Module: crate::punctuated
// Provides: {"Punctuated"}
// Dependencies: {}
# [doc = " **A punctuated sequence of syntax tree nodes of type `T` separated by"] # [doc = " punctuation of type `P`.**"] # [doc = ""] # [doc = " Refer to the [module documentation] for details about punctuated sequences."] # [doc = ""] # [doc = " [module documentation]: self"] pub struct Punctuated < T , P > { inner : Vec < (T , P) > , last : Option < Box < T > > , }
};
}
