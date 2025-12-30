// Generated macro for IanaNames (struct)
macro_rules! Depcrate_provider_ianaIanaNames {
() => {
// Module: crate::provider::iana
// Provides: {"IanaNames"}
// Dependencies: {}
# [doc = " A mapping from IANA time zone identifiers to BCP-47 time zone identifiers."] # [doc = ""] # [doc = " The BCP-47 time zone ID maps to the default IANA time zone ID according to the CLDR data."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , Clone , PartialEq , zerofrom :: ZeroFrom , yoke :: Yokeable)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_time :: provider :: iana))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [yoke (prove_covariance_manually)] pub struct IanaNames < 'data > { # [doc = " The list of all normalized IANA identifiers."] # [doc = ""] # [doc = " The first `bcp47_ids.len()` identifiers are canonical for the"] # [doc = " the BCP-47 IDs in [`IanaToBcp47Map::bcp47_ids`] at the same index."] # [doc = ""] # [doc = " The remaining non-canonical identifiers are sorted in ascending lowercase order."] # [cfg_attr (feature = "serde" , serde (borrow))] pub normalized_iana_ids : VarZeroVec < 'data , str > , }
};
}
