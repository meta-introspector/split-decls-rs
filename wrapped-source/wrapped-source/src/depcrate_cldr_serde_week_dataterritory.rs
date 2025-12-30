// Generated macro for Territory (enum)
macro_rules! Depcrate_cldr_serde_week_dataTerritory {
() => {
// Module: crate::cldr_serde::week_data
// Provides: {"Territory"}
// Dependencies: {}
# [doc = " The territory that data is keyed by."] # [doc = ""] # [doc = " For example the \"AD\" in \"weekData\": { \"minDays\": { \"AD\": 4, } }"] # [doc = ""] # [doc = " The contained types are strings rather than [`icu::locale::subtags::Region`]"] # [doc = " to avoid an extra parsing step of the variant in data providers."] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub (crate) enum Territory { Region (Region) , AltVariantRegion (Region) , }
};
}
