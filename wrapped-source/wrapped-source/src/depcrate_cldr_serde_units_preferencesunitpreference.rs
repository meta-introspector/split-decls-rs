// Generated macro for UnitPreference (struct)
macro_rules! Depcrate_cldr_serde_units_preferencesUnitPreference {
() => {
// Module: crate::cldr_serde::units::preferences
// Provides: {"UnitPreference"}
// Dependencies: {}
# [doc = " Represents a single unit preference entry with an optional threshold"] # [derive (PartialEq , Debug , Deserialize)] pub (crate) struct UnitPreference { # [doc = " The unit identifier (e.g., \"square-kilometer\", \"hectare\")"] pub (crate) unit : String , # [doc = " Optional threshold value - unit is preferred when the value is greater than or equal to this"] # [serde (rename = "geq")] pub (crate) greater_or_equal : Option < f64 > , }
};
}
