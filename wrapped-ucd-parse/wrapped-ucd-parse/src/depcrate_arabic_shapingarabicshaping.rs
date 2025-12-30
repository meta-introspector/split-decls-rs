// Generated macro for ArabicShaping (struct)
macro_rules! Depcrate_arabic_shapingArabicShaping {
() => {
// Module: crate::arabic_shaping
// Provides: {"ArabicShaping"}
// Dependencies: {}
# [doc = " Represents a single row in the `ArabicShaping.txt` file."] # [doc = ""] # [doc = " The field names were taken from the header of ArabicShaping.txt."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct ArabicShaping { # [doc = " The codepoint corresponding to this row."] pub codepoint : Codepoint , # [doc = " A short schematic name for the codepoint."] # [doc = ""] # [doc = " The schematic name is descriptive of the shape, based as consistently as"] # [doc = " possible on a name for the skeleton and then the diacritic marks applied"] # [doc = " to the skeleton, if any.  Note that this schematic name is considered a"] # [doc = " comment, and does not constitute a formal property value."] pub schematic_name : String , # [doc = " The \"joining type\" of this codepoint."] pub joining_type : JoiningType , # [doc = " The \"joining group\" of this codepoint."] pub joining_group : String , }
};
}
