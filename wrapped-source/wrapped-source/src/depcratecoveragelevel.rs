// Generated macro for CoverageLevel (enum)
macro_rules! DepcrateCoverageLevel {
() => {
// Module: crate
// Provides: {"CoverageLevel"}
// Dependencies: {}
# [doc = " A language's CLDR coverage level."] # [doc = ""] # [doc = " In ICU4X, these are disjoint sets: a language belongs to a single coverage level. This"] # [doc = " contrasts with CLDR usage, where these levels are understood to be additive (i.e., \"basic\""] # [doc = " includes all language with \"basic\", or better coverage). The ICU4X semantics allow"] # [doc = " generating different data files for different coverage levels without duplicating data."] # [doc = " However, the data itself is still additive (e.g. for fallback to work correctly), so data"] # [doc = " for moderate (basic) languages should only be loaded if modern (modern and moderate) data"] # [doc = " is already present."] # [derive (Debug , Copy , Clone , PartialEq , Eq , serde :: Deserialize , serde :: Serialize , Hash)] # [non_exhaustive] # [serde (rename_all = "camelCase")] pub enum CoverageLevel { # [doc = " Locales listed as modern coverage targets by the CLDR subcomittee."] # [doc = ""] # [doc = " This is the highest level of coverage."] Modern , # [doc = " Locales listed as moderate, but not modern, coverage targets by the CLDR subcomittee."] # [doc = ""] # [doc = " This is a medium level of coverage."] Moderate , # [doc = " Locales listed as basic, but not moderate or modern, coverage targets by the CLDR subcomittee."] # [doc = ""] # [doc = " This is the lowest level of coverage."] Basic , }
};
}
