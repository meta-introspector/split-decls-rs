// Generated macro for Supplemental (struct)
macro_rules! Depcrate_cldr_serde_units_preferencesSupplemental {
() => {
// Module: crate::cldr_serde::units::preferences
// Provides: {"Supplemental"}
// Dependencies: {}
# [doc = " The supplemental data containing unit preferences"] # [derive (PartialEq , Debug , Deserialize)] pub (crate) struct Supplemental { # [doc = " Version information for the CLDR data"] # [serde (rename = "version")] pub (crate) version : Option < BTreeMap < String , String > > , # [doc = " The main unit preference data organized by category -> usage -> region -> preferences"] # [serde (rename = "unitPreferenceData")] pub (crate) unit_preference_data : CategoryPreferences , }
};
}
