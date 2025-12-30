// Generated macro for convert_slices_to_fraction (function)
macro_rules! Depcrate_units_helpersconvert_slices_to_fraction {
() => {
// Module: crate::units::helpers
// Provides: {"convert_slices_to_fraction"}
// Dependencies: {}
# [doc = " Converts slices of numerator and denominator strings to a fraction."] # [doc = " Examples:"] # [doc = " - [\"1\"], [\"2\"] is converted to 1/2"] # [doc = " - [\"1\", \"2\"], [\"3\", \"1E2\"] is converted to 1*2/(3*1E2) --> 2/300"] # [doc = " - [\"1\", \"2\"], [\"3\", \"1E-2\"] is converted to 1*2/(3*1E-2) --> 200/3"] # [doc = " - [\"1\", \"2\"], [\"3\", \"1E-2.5\"] is an invalid scientific notation number"] # [doc = " - [\"1E2\"], [\"2\"] is converted to 1E2/2 --> 100/2 --> 50/1"] # [doc = " - [\"1E2\", \"2\"], [\"3\", \"1E2\"] is converted to 1E2*2/(3*1E2) --> 2/3"] pub (crate) fn convert_slices_to_fraction (numerator_strings : & [String] , denominator_strings : & [String] ,) -> Result < IcuRatio , DataError > { numerator_strings . iter () . try_fold (IcuRatio :: one () , | result , num | { IcuRatio :: from_str (num . as_str ()) . map_err (| _ | { DataError :: custom ("The numerator is not a valid scientific notation number") }) . map (| num_fraction | result * num_fraction) }) . and_then (| num_product | { denominator_strings . iter () . try_fold (num_product , | result , den | { IcuRatio :: from_str (den . as_str ()) . map_err (| _ | { DataError :: custom ("The denominator is not a valid scientific notation number" ,) }) . map (| den_fraction | result / den_fraction) }) }) }
};
}
