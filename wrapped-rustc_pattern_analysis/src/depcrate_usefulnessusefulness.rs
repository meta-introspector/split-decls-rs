// Generated macro for Usefulness (enum)
macro_rules! Depcrate_usefulnessUsefulness {
() => {
// Module: crate::usefulness
// Provides: {"Usefulness"}
// Dependencies: {}
# [doc = " Indicates whether or not a given arm is useful."] # [derive (Clone , Debug)] pub enum Usefulness < 'p , Cx : PatCx > { # [doc = " The arm is useful. This additionally carries a set of or-pattern branches that have been"] # [doc = " found to be redundant despite the overall arm being useful. Used only in the presence of"] # [doc = " or-patterns, otherwise it stays empty."] Useful (Vec < (& 'p DeconstructedPat < Cx > , RedundancyExplanation < 'p , Cx >) >) , # [doc = " The arm is redundant and can be removed without changing the behavior of the match"] # [doc = " expression."] Redundant (RedundancyExplanation < 'p , Cx >) , }
};
}
