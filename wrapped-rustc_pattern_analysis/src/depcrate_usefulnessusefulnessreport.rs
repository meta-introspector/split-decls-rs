// Generated macro for UsefulnessReport (struct)
macro_rules! Depcrate_usefulnessUsefulnessReport {
() => {
// Module: crate::usefulness
// Provides: {"UsefulnessReport"}
// Dependencies: {}
# [doc = " The output of checking a match for exhaustiveness and arm usefulness."] pub struct UsefulnessReport < 'p , Cx : PatCx > { # [doc = " For each arm of the input, whether that arm is useful after the arms above it."] pub arm_usefulness : Vec < (MatchArm < 'p , Cx > , Usefulness < 'p , Cx >) > , # [doc = " If the match is exhaustive, this is empty. If not, this contains witnesses for the lack of"] # [doc = " exhaustiveness."] pub non_exhaustiveness_witnesses : Vec < WitnessPat < Cx > > , # [doc = " For each arm, a set of indices of arms above it that have non-empty intersection, i.e. there"] # [doc = " is a value matched by both arms. This may miss real intersections."] pub arm_intersections : Vec < DenseBitSet < usize > > , }
};
}
