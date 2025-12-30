// Generated macro for TrackElem (enum)
macro_rules! Depcrate_value_analysisTrackElem {
() => {
// Module: crate::value_analysis
// Provides: {"TrackElem"}
// Dependencies: {}
# [doc = " The set of projection elements that can be used by a tracked place."] # [doc = ""] # [doc = " Although only field projections are currently allowed, this could change in the future."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum TrackElem { Field (FieldIdx) , Variant (VariantIdx) , Discriminant , DerefLen , }
};
}
