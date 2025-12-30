// Generated macro for ListPatterns (struct)
macro_rules! Depcrate_cldr_serde_list_patternsListPatterns {
() => {
// Module: crate::cldr_serde::list_patterns
// Provides: {"ListPatterns"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct ListPatterns { # [serde (rename = "listPattern-type-standard")] pub (crate) standard : ListPattern , # [serde (rename = "listPattern-type-standard-narrow")] pub (crate) standard_narrow : ListPattern , # [serde (rename = "listPattern-type-standard-short")] pub (crate) standard_short : ListPattern , # [serde (rename = "listPattern-type-or")] pub (crate) or : ListPattern , # [serde (rename = "listPattern-type-or-narrow")] pub (crate) or_narrow : ListPattern , # [serde (rename = "listPattern-type-or-short")] pub (crate) or_short : ListPattern , # [serde (rename = "listPattern-type-unit")] pub (crate) unit : ListPattern , # [serde (rename = "listPattern-type-unit-narrow")] pub (crate) unit_narrow : ListPattern , # [serde (rename = "listPattern-type-unit-short")] pub (crate) unit_short : ListPattern , }
};
}
