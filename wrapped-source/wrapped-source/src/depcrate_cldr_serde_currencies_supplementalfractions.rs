// Generated macro for Fractions (struct)
macro_rules! Depcrate_cldr_serde_currencies_supplementalFractions {
() => {
// Module: crate::cldr_serde::currencies::supplemental
// Provides: {"Fractions"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct Fractions { # [serde (rename = "DEFAULT")] default : RoundingModes , # [serde (flatten)] pub (crate) currencies : BTreeMap < ISOCode , RoundingModes > , }
};
}
