// Generated macro for MatchArm (struct)
macro_rules! DepcrateMatchArm {
() => {
// Module: crate
// Provides: {"MatchArm"}
// Dependencies: {}
# [doc = " The arm of a match expression."] # [derive (Debug)] pub struct MatchArm < 'p , Cx : PatCx > { pub pat : & 'p DeconstructedPat < Cx > , pub has_guard : bool , pub arm_data : Cx :: ArmData , }
};
}
