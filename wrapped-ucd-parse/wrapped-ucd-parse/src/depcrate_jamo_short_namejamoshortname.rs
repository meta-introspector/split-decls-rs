// Generated macro for JamoShortName (struct)
macro_rules! Depcrate_jamo_short_nameJamoShortName {
() => {
// Module: crate::jamo_short_name
// Provides: {"JamoShortName"}
// Dependencies: {}
# [doc = " A single row in the `Jamo.txt` file."] # [doc = ""] # [doc = " The `Jamo.txt` file defines the `Jamo_Short_Name` property."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct JamoShortName { # [doc = " The codepoint corresponding to this row."] pub codepoint : Codepoint , # [doc = " The actual \"Jamo Short Name.\" This string contains at most 3 bytes and"] # [doc = " may be empty."] pub name : String , }
};
}
