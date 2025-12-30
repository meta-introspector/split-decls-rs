// Generated macro for VariantOffsets (struct)
macro_rules! Depcrate_providerVariantOffsets {
() => {
// Module: crate::provider
// Provides: {"VariantOffsets"}
// Dependencies: {}
# [doc = " Represents the different offsets in use for a time zone"] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , PartialOrd , Eq , Ord)] pub struct VariantOffsets { # [doc = " The standard offset."] pub standard : UtcOffset , # [doc = " The daylight-saving offset, if used."] pub daylight : Option < UtcOffset > , }
};
}
