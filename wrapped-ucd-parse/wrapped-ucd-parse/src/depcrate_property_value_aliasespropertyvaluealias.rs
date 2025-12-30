// Generated macro for PropertyValueAlias (struct)
macro_rules! Depcrate_property_value_aliasesPropertyValueAlias {
() => {
// Module: crate::property_value_aliases
// Provides: {"PropertyValueAlias"}
// Dependencies: {}
# [doc = " A single row in the `PropertyValueAliases.txt` file."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct PropertyValueAlias { # [doc = " The property name for which this value alias applies."] pub property : String , # [doc = " A numeric abbreviation for this property value, if present. (This is"] # [doc = " seemingly only present for the `ccc`/`Canonical_Combining_Class`"] # [doc = " property.)"] pub numeric : Option < u8 > , # [doc = " An abbreviation for this property value."] pub abbreviation : String , # [doc = " The \"long\" form of this property value."] pub long : String , # [doc = " Additional value aliases (if present)."] pub aliases : Vec < String > , }
};
}
