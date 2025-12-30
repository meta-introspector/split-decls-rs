// Generated macro for DerivedNumericValues (struct)
macro_rules! Depcrate_extracted_derived_numeric_valuesDerivedNumericValues {
() => {
// Module: crate::extracted::derived_numeric_values
// Provides: {"DerivedNumericValues"}
// Dependencies: {}
# [doc = " A single row in the `extracted/DerivedNumericValues.txt` file."] # [doc = ""] # [doc = " This file gives the derived values of the Numeric_Value property."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct DerivedNumericValues { # [doc = " The codepoint or codepoint range for this entry."] pub codepoints : Codepoints , # [doc = " The approximate Numeric_Value of the codepoints in this entry,"] # [doc = " as a decimal."] pub numeric_value_decimal : String , # [doc = " The exact Numeric_Value of the codepoints in this entry, as"] # [doc = " a fraction."] pub numeric_value_fraction : String , }
};
}
