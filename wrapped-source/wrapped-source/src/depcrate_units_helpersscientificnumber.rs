// Generated macro for ScientificNumber (struct)
macro_rules! Depcrate_units_helpersScientificNumber {
() => {
// Module: crate::units::helpers
// Provides: {"ScientificNumber"}
// Dependencies: {}
# [doc = " Represents a scientific number that contains only clean numerator and denominator terms."] # [doc = " NOTE:"] # [doc = "   clean means that there is no constant in the numerator or denominator."] # [doc = "   For example, [\"1.2\"] is clean, but [\"1.2\", ft_to_m\"] is not clean."] pub (crate) struct ScientificNumber { # [doc = " Contains numerator terms that are represented as scientific numbers"] pub (crate) clean_num : Vec < String > , # [doc = " Contains denominator terms that are represented as scientific numbers"] pub (crate) clean_den : Vec < String > , # [doc = " Indicates if the constant is exact or approximate"] pub (crate) exactness : Exactness , }
};
}
