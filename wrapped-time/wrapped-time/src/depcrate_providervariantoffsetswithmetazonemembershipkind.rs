// Generated macro for VariantOffsetsWithMetazoneMembershipKind (struct)
macro_rules! Depcrate_providerVariantOffsetsWithMetazoneMembershipKind {
() => {
// Module: crate::provider
// Provides: {"VariantOffsetsWithMetazoneMembershipKind"}
// Dependencies: {}
# [doc = " A [`VariantOffsets`] and a [`MetazoneMembershipKind`] packed into one byte."] # [derive (Debug , Clone , Copy , PartialEq , PartialOrd , Eq , Ord)] pub struct VariantOffsetsWithMetazoneMembershipKind { # [doc = " The offsets. Currently uses 3 bits."] pub offsets : VariantOffsets , # [doc = " Metazone membership metadata. Currently uses 2 bits."] pub mzmsk : MetazoneMembershipKind , }
};
}
