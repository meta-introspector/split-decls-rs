// Generated macro for UnitsLengthData (struct)
macro_rules! Depcrate_cldr_serde_units_dataUnitsLengthData {
() => {
// Module: crate::cldr_serde::units::data
// Provides: {"UnitsLengthData"}
// Dependencies: {}
# [derive (PartialEq , Debug)] pub (crate) struct UnitsLengthData { # [doc = " Maps from each category to a map for each units with their associated patterns"] pub (crate) categories : BTreeMap < String , BTreeMap < String , Patterns > > , pub (crate) per : Patterns , pub (crate) times : Patterns , pub (crate) powers : BTreeMap < usize , Patterns > , pub (crate) binary : BTreeMap < u8 , Patterns > , pub (crate) decimal : BTreeMap < i8 , Patterns > , }
};
}
