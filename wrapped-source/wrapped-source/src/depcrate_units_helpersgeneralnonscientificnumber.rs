// Generated macro for GeneralNonScientificNumber (struct)
macro_rules! Depcrate_units_helpersGeneralNonScientificNumber {
() => {
// Module: crate::units::helpers
// Provides: {"GeneralNonScientificNumber"}
// Dependencies: {}
# [doc = " Represents a general constant which contains scientific and non scientific numbers."] # [derive (Debug)] struct GeneralNonScientificNumber { # [doc = " Contains numerator terms that are represented as scientific numbers"] clean_num : Vec < String > , # [doc = " Contains denominator terms that are represented as scientific numbers"] clean_den : Vec < String > , # [doc = " Contains numerator terms that are not represented as scientific numbers"] non_scientific_num : VecDeque < String > , # [doc = " Contains denominator terms that are not represented as scientific numbers"] non_scientific_den : VecDeque < String > , # [doc = " Indicates if the constant is exact or approximate"] exactness : Exactness , }
};
}
