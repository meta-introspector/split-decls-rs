// Generated macro for Resource (struct)
macro_rules! Depcrate_cldr_serde_coverage_levelsResource {
() => {
// Module: crate::cldr_serde::coverage_levels
// Provides: {"Resource"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct Resource { # [serde (rename = "coverageLevels")] pub (crate) coverage_levels : HashMap < icu :: locale :: LanguageIdentifier , crate :: CoverageLevel > , }
};
}
