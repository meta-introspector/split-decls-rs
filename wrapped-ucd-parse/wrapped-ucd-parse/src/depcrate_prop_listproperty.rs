// Generated macro for Property (struct)
macro_rules! Depcrate_prop_listProperty {
() => {
// Module: crate::prop_list
// Provides: {"Property"}
// Dependencies: {}
# [doc = " A single row in the `PropList.txt` file."] # [doc = ""] # [doc = " The `PropList.txt` file is the source of truth on several Unicode"] # [doc = " properties."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct Property { # [doc = " The codepoint or codepoint range for this entry."] pub codepoints : Codepoints , # [doc = " The property name assigned to the codepoints in this entry."] pub property : String , }
};
}
