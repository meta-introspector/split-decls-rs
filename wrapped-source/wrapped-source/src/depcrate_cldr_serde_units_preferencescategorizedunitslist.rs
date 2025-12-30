// Generated macro for CategorizedUnitsList (type)
macro_rules! Depcrate_cldr_serde_units_preferencesCategorizedUnitsList {
() => {
// Module: crate::cldr_serde::units::preferences
// Provides: {"CategorizedUnitsList"}
// Dependencies: {}
# [doc = " Maps a category (e.g., \"length\", \"mass\", \"duration\") to a map from each region to a set of units."] # [doc = " This excludes usages; it only checks if the unit is present for a given country in a specific category and region."] pub (crate) type CategorizedUnitsList = BTreeMap < String , BTreeMap < String , HashSet < String > > > ;
};
}
