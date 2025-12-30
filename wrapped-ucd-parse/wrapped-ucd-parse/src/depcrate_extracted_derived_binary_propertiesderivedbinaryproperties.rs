// Generated macro for DerivedBinaryProperties (struct)
macro_rules! Depcrate_extracted_derived_binary_propertiesDerivedBinaryProperties {
() => {
// Module: crate::extracted::derived_binary_properties
// Provides: {"DerivedBinaryProperties"}
// Dependencies: {}
# [doc = " A single row in the `extracted/DerivedBinaryProperties.txt` file."] # [doc = ""] # [doc = " This file indicates whether a codepoint has the Bidi_Mirrored property."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct DerivedBinaryProperties { # [doc = " The codepoint or codepoint range for this entry."] pub codepoints : Codepoints , # [doc = " The derived property of the codepoints in this entry. Currently,"] # [doc = " this is always the always the string \"Bidi_Mirrored\"."] pub property : String , }
};
}
