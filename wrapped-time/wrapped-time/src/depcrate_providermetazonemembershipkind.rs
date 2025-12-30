// Generated macro for MetazoneMembershipKind (enum)
macro_rules! Depcrate_providerMetazoneMembershipKind {
() => {
// Module: crate::provider
// Provides: {"MetazoneMembershipKind"}
// Dependencies: {}
# [doc = " Metadata about a metazone membership"] # [derive (Debug , Clone , Copy , PartialEq , PartialOrd , Eq , Ord)] # [non_exhaustive] pub enum MetazoneMembershipKind { # [doc = " This zone is equivalent to the metazone's golden time zone."] BehavesLikeGolden , # [doc = " This zone uses variants that the golden zone does not use."] # [doc = " This happens for example for London, Dublin, Troll (all in GMT), Windhoek (in WAT)."] CustomVariants , # [doc = " This zone uses different transitions than the golden zone."] # [doc = " This happens for example for Phoenix, Regina, Algiers, Brisbane (no DST),"] # [doc = " or Chisinau (transitions at different times, not implemented yet)."] CustomTransitions , }
};
}
