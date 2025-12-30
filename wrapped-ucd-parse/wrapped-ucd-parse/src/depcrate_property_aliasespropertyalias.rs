// Generated macro for PropertyAlias (struct)
macro_rules! Depcrate_property_aliasesPropertyAlias {
() => {
// Module: crate::property_aliases
// Provides: {"PropertyAlias"}
// Dependencies: {}
# [doc = " A single row in the `PropertyAliases.txt` file."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct PropertyAlias { # [doc = " An abbreviation for this property."] pub abbreviation : String , # [doc = " The \"long\" name of this property."] pub long : String , # [doc = " Additional aliases (if present)."] pub aliases : Vec < String > , }
};
}
