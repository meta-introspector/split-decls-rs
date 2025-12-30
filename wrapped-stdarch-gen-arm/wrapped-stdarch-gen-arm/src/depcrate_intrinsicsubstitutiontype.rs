// Generated macro for SubstitutionType (enum)
macro_rules! Depcrate_intrinsicSubstitutionType {
() => {
// Module: crate::intrinsic
// Provides: {"SubstitutionType"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] # [serde (untagged)] pub enum SubstitutionType { MatchSize (SizeMatchable < WildString >) , MatchKind (KindMatchable < WildString >) , }
};
}
